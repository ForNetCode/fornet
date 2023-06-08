package com.timzaak.fornet.mqtt

import com.timzaak.fornet.dao.*
import com.timzaak.fornet.entity.PublicKey
import com.timzaak.fornet.grpc.convert.EntityConvert
import com.timzaak.fornet.mqtt.api.RMqttApiClient
import com.timzaak.fornet.protobuf.config.ClientMessage
import com.timzaak.fornet.pubsub.MqttConnectionManager
import com.timzaak.fornet.service.NodeService
import com.typesafe.config.Config
import inet.ipaddr.IPAddress.IPVersion
import inet.ipaddr.IPAddressString
import inet.ipaddr.ipv4.IPv4Address
import org.hashids.Hashids
import org.scalatra.{BadRequest, Forbidden, Ok, ScalatraServlet}
import very.util.web.LogSupport
import very.util.web.json.{JsonResponse, ZIOJsonSupport}
import very.util.web.validate.ValidationExtra
import zio.json.{DeriveJsonDecoder, JsonDecoder, jsonField}

import scala.tools.nsc.backend.jvm.BackendReporting.Invalid
import scala.util.{Failure, Try}

case class AuthRequest(
  clientId: String, // publicKey
  username: String, // nodeHashId
  password: String, // |nonce|timestamp|signature
)

given JsonDecoder[AuthRequest] = DeriveJsonDecoder.gen

case class WebHookCallbackRequest(
  action: String,
  @jsonField("clientid")
  clientId: String,
  topic: String,
)
given JsonDecoder[WebHookCallbackRequest] = DeriveJsonDecoder.gen

case class AclRequest(
  // 1 = sub, 2 = pub
  access: String,
  username: String,
  ipaddr: String,
  @jsonField("clientid")
  clientId: String,
  topic: String
)

given JsonDecoder[AclRequest] = DeriveJsonDecoder.gen

class MqttCallbackController(
  nodeDao: NodeDao,
  networkDao: NetworkDao,
  nodeService: NodeService,
  mqttConnectionManager: MqttConnectionManager,
)(using hashId: Hashids)
  extends ScalatraServlet
  with LogSupport
  with ZIOJsonSupport {

  jPost("/auth") { (req: AuthRequest) =>
    import req.*
    val data = password.split('|')
    val isOk = if (data.length != 3) {
      val signature = data.last
      val plainText = data.dropRight(1).mkString("-")
      PublicKey(clientId).validate(plainText, signature) && nodeDao
        .findByPublicKey(clientId)
        .nonEmpty
    } else {
      false
    }
    if (isOk) {
      Ok()
    } else {
      Forbidden()
    }

  }

  jPost("/webhook") { (req: WebHookCallbackRequest) =>
    logger.debug(s"mqtt hook: ${request.body}")
    import req.*

    // {"action":"client_subscribe","clientid":"C5yG28uwzTumy6PpBEGqvvEWLJ8dYzF1uSFGziJG6Q8Jl+DPCRZZX05MPXb/s9GWsuO2JXzADAHz70WVbD2lew==","ipaddress":"127.0.0.1:56588","node":1,"opts":{"qos":1},"topic":"client","username":"undefined"}

    Try {
      if (action == "client_subscribe" && topic == "client") {
        // send wr config
        val nodes =
          nodeDao
            .findByPublicKey(clientId)
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
            mqttConnectionManager.sendMessage(
              networkId = node.networkId,
              node.id,
              clientId,
              ClientMessage(
                networkId = node.networkId.secretId,
                ClientMessage.Info.Config(
                  EntityConvert.nodeToWRConfig(node, network, notifyNodes)
                )
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

  post("/superuser") {
    logger.debug(s"mqtt super user does not implement ${request.body}")
    Forbidden()
  }

  jPost("/acl") { (req: AclRequest) =>
    // logger.debug(s"mqtt acl does not implement,body: ${request.body}")
    // pub
    if (req.access == "2") {
      val isPrivateIP =
        Try(IPAddressString(req.ipaddr).toAddress(IPVersion.IPV4).asInstanceOf[IPv4Address].isPrivate) match {
          case scala.util.Success(v) => v
          case _                     => false
        }
      if (isPrivateIP) {
        Ok()
      } else {
        Forbidden()
      }
      // sub
    } else if (req.access == "1") {

      // TODO: check it can only subscribe self, add hashId to username,and check
    }
    Ok()
  }
}
