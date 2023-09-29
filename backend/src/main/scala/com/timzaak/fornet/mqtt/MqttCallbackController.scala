package com.timzaak.fornet.mqtt

import com.timzaak.fornet.dao.*
import com.timzaak.fornet.entity.PublicKey
import com.timzaak.fornet.grpc.convert.EntityConvert
import com.timzaak.fornet.protobuf.config.ClientMessage
import com.timzaak.fornet.pubsub.MqttConnectionManager
import com.timzaak.fornet.service.NodeService
import com.typesafe.scalalogging.LazyLogging
import inet.ipaddr.IPAddress.IPVersion
import inet.ipaddr.IPAddressString
import inet.ipaddr.ipv4.IPv4Address
import org.hashids.Hashids
import org.scalatra.*
import very.util.security.ID.{ toIntID, toTokenID }
import very.util.security.TokenID
import very.util.web.json.ZIOJsonSupport
import very.util.web.validate.ValidationExtra
import zio.json.{ DeriveJsonDecoder, JsonDecoder, jsonField }

import scala.util.matching.Regex
import scala.util.{ Failure, Success, Try }

import very.util.web.json.tokenIDDecoder

case class AuthRequest(
  @jsonField("clientId")
  deviceTokenId: TokenID, // device tokenId
  @jsonField("username")
  publicKey: String, // public key
  password: String, // |nonce|timestamp|signature
)

given authRequestDecoder(using hashId: Hashids): JsonDecoder[AuthRequest] =
  DeriveJsonDecoder.gen

case class WebHookCallbackRequest(
  action: String,
  @jsonField("clientid")
  deviceTokenId: TokenID,
  topic: String,
  @jsonField("username")
  publicKey: String,
)
given webHookCallbackRequestDecoder(using
  hashId: Hashids
): JsonDecoder[WebHookCallbackRequest] = DeriveJsonDecoder.gen

case class AclRequest(
  // 1 = sub, 2 = pub
  access: String,
  @jsonField("username")
  publicKey: String,
  ipaddr: String,
  @jsonField("clientId")
  deviceTokenId: TokenID,
  topic: String
)

given aclRequestDecoder(using hashId: Hashids): JsonDecoder[AclRequest] =
  DeriveJsonDecoder.gen

private val networkTopicPattern = """^network/(\w+)$""".r
private val clientTopicPattern = """^client/(\w+)$""".r

class MqttCallbackController(
  nodeDao: NodeDao,
  deviceDao: DeviceDao,
  networkDao: NetworkDao,
  nodeService: NodeService,
  mqttConnectionManager: MqttConnectionManager,
)(using hashId: Hashids)
  extends ScalatraServlet
  with LazyLogging
  with ZIOJsonSupport {

  jPost("/auth") { (req: AuthRequest) =>
    import req.*
    val data = password.split('|')
    val isOk = if (data.length == 3) {
      val signature = data.last
      val plainText =
        s"${deviceTokenId.secretId}|${data.dropRight(1).mkString("|")}"
      PublicKey(publicKey).validate(plainText, signature) && deviceDao
        .findByTokenID(deviceTokenId)
        .exists(_.publicKey == publicKey)
    } else {
      false
    }
    logger.debug(s"device: ${req.deviceTokenId.id} auth ${isOk}")
    if (isOk) {
      Ok("allow")
    } else {
      Ok("deny")
    }

  }

  jPost("/webhook") { (req: WebHookCallbackRequest) =>
    logger.debug(s"mqtt hook: ${request.body}")
    import req.*
    // {"action":"client_subscribe","clientid":"xxx","ipaddress":"127.0.0.1:56588","node":1,"opts":{"qos":1},"topic":"client","username":"undefined"}
    Try {
      if (
        action == "client_subscribe" && topic == s"client/${req.deviceTokenId.secretId}"
      ) {
        // send wr config
        val nodes =
          nodeDao
            .findByDeviceId(deviceTokenId)
            .filter(_.status == NodeStatus.Normal)

        val networks = if (nodes.isEmpty) {
          Map.empty
        } else {
          networkDao
            .findByIds(nodes.map(_.networkId).distinct)
            .map(v => v.id -> v)
            .toMap
        }

        nodes.foreach { node =>
          val network: Network = networks(node.networkId)
          if (node.realStatus(network.status) == NodeStatus.Normal) {
            val notifyNodes = nodeService.getAllRelativeNodes(node)
            val network = networks(node.networkId)
            val deviceMap = deviceDao.getAllDevices(node.deviceId::notifyNodes.map(_.deviceId))
            mqttConnectionManager.sendClientMessage(
              networkId = node.networkId,
              deviceTokenId,
              ClientMessage(
                networkId = node.networkId.secretId,
                ClientMessage.Info.Config(
                  EntityConvert
                    .nodeToWRConfig(node, network, notifyNodes, deviceMap)
                ),
              )
            )
          }
        }
      }
    } match {
      case Failure(e) => logger.warn("mqtt handle failure:", e)
      case _          => // do nothing
    }
    Ok()
  }

  jPost("/acl") { (req: AclRequest) =>
    logger.debug(s"mqtt acl: ${request.body}")
    // pub
    val result: ActionResult = if (req.access == "2") {
      val isPrivateIP =
        Try(
          IPAddressString(req.ipaddr)
            .toAddress(IPVersion.IPV4)
            .asInstanceOf[IPv4Address]
            .isPrivate
        ) match {
          case Success(v) => v
          case _          => false
        }
      if (isPrivateIP) {
        Ok("allow")
      } else {
        Ok("deny")
      }
      // sub
    } else if (req.access == "1") {
      req.topic match {
        case networkTopicPattern(secretId) =>
          Try(secretId.toTokenID).fold(
            _ => Ok("deny"),
            { networkId =>
              if (
                nodeDao
                  .findByDeviceWithNetwork(networkId.intId, req.deviceTokenId)
                  .nonEmpty
              ) {
                Ok("allow")
              } else {
                Ok("deny")
              }
            }
          )
        case clientTopicPattern(secretId)
          if secretId == req.deviceTokenId.secretId =>
          Ok("allow")
        case _ =>
          Ok("deny")
      }
    } else {
      Ok("deny")
    }
    result
  }
}
