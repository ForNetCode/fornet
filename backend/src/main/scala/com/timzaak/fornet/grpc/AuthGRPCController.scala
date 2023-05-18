package com.timzaak.fornet.grpc

import ch.qos.logback.core.joran.action.Action
import com.google.common.base.Charsets
import com.google.protobuf.empty.Empty
import com.timzaak.fornet.controller.auth.AppAuthStrategyProvider
import com.timzaak.fornet.dao.*
import com.timzaak.fornet.protobuf.auth.*
import com.timzaak.fornet.pubsub.NodeChangeNotifyService
import com.timzaak.fornet.service.{GRPCAuth, NodeAuthService}
import com.typesafe.config.Config
import com.typesafe.scalalogging.Logger
import inet.ipaddr.IPAddress.IPVersion
import inet.ipaddr.IPAddressString
import inet.ipaddr.ipv4.IPv4Address
import io.getquill.*
import org.hashids.Hashids
import very.util.keycloak.KeycloakJWTAuthStrategy
import very.util.web.LogSupport
import zio.json.*
import zio.json.ast.{Json, JsonCursor}

import java.net.http.HttpRequest.BodyPublishers
import java.net.http.{HttpClient, HttpRequest}
import java.net.{URI, URLEncoder}
import java.time.{LocalDateTime, OffsetDateTime}
import scala.concurrent.Future

class AuthGRPCController(
  hashId: Hashids,
  nodeDao: NodeDao,
  networkDao: NetworkDao,
  nodeChangeNotifyService: NodeChangeNotifyService,
  config: Config,
  nodeAuthService: NodeAuthService,
  authStrategyProvider: AppAuthStrategyProvider,
)(using quill: DB)
  extends AuthGrpc.Auth
  with LogSupport {

  import very.util.config.get
  private val mqttClientUrl = config.get[String]("mqtt.clientUrl")

  import quill.{*, given}

  override def inviteConfirm(
    request: InviteConfirmRequest
  ): Future[ActionResponse] = {
    var params = Seq(request.networkId)
    if (request.nodeId.nonEmpty) {
      params = params.appended(request.nodeId.get)
    }
    if (request.encrypt.exists(v => nodeAuthService.validate(v, params))) {
      val networkId = hashId.decode(request.networkId).head.toInt
      val publicKey = request.encrypt.get.publicKey

      val response = request.nodeId match {
        case Some(nodeIdStr) =>
          // confirm node exists and change status
          val nodeId = hashId.decode(nodeIdStr).head.toInt

          val changeCount = quill.run {
            quote {
              query[Node]
                .filter(n =>
                  n.networkId == lift(networkId) && n.id == lift(
                    nodeId
                  ) && n.status == lift(NodeStatus.Waiting)
                )
                .update(
                  _.status -> lift(NodeStatus.Normal),
                  _.publicKey -> lift(request.encrypt.get.publicKey),
                  _.updatedAt -> lift(OffsetDateTime.now()),
                )
            }
          }

          if (changeCount > 0) {
            // notify relay node online
            val node = nodeDao.findById(networkId, nodeId).get
            nodeChangeNotifyService.nodeStatusChangeNotify(
              node,
              NodeStatus.Waiting,
              NodeStatus.Normal
            )
            ActionResponse(true, mqttUrl = Some(mqttClientUrl))
          } else {
            ActionResponse(message = Some("already active or error response"))
          }
        case None =>
          createNode(networkId, publicKey) match {
            case Some(value) => ActionResponse(message = Some(value))
            case None =>
              ActionResponse(isOk = true, mqttUrl = Some(mqttClientUrl))
          }
      }
      Future.successful(response)
    } else {
      Future.successful(
        ActionResponse(message = Some("Illegal Arguments"))
      )
    }
  }

  import very.util.config.get

  override def oauthDeviceCodeConfirm(
    request: OAuthDeviceCodeRequest
  ): Future[ActionResponse] = {
    val params = Seq(request.accessToken, request.deviceCode, request.networkId)
    if (request.encrypt.exists(v => nodeAuthService.validate(v, params))) {

      if (config.hasPath("auth.keycloak")) {
        val authResult = authStrategyProvider
          .getStrategy(KeycloakJWTAuthStrategy.name)
          .flatMap { auth => auth.auth(request.accessToken) }
          .toRight("auth process error, please check server error log")

        authResult match {
          case Left(value) =>
            Future.successful(ActionResponse(message = Some(value)))
          case Right(userId) =>
            val publicKey = request.encrypt.get.publicKey
            val networkId = hashId.decode(request.networkId).head.toInt

            logger.info(
              s"user:${userId},networkId:${networkId}, publicKey:${request.encrypt.get.publicKey} register device with code:${request.deviceCode}"
            )
            Future.successful(
              createNode(networkId, publicKey) match {
                case Some(value) => ActionResponse(message = Some(value))
                case None =>
                  ActionResponse(isOk = true, mqttUrl = Some(mqttClientUrl))
              }
            )
        }
      } else {
        Future.successful(
          ActionResponse(message = Some("do not support keycloak now"))
        )
      }
    } else {
      Future.successful(
        ActionResponse(message = Some("Illegal Arguments"))
      )
    }
  }

  private def createNode(networkId: Int, publicKey: String) = {
    val network = networkDao.findById(networkId).get
    // network create node
    val usedIp = nodeDao
      .getUsedIps(network.id)
      .map(ip =>
        IPAddressString(ip)
          .toAddress(IPVersion.IPV4)
          .asInstanceOf[IPv4Address]
          .intValue()
      )
      .toSet
    val addressRange = IPAddressString(network.addressRange)
      .toAddress(IPVersion.IPV4)
      .toPrefixBlock()
      .asInstanceOf[IPv4Address]
    (addressRange.getLowerNonZeroHost
      .intValue() to addressRange.getUpper.intValue())
      .find(!usedIp.contains(_)) match {
      case Some(ip) =>
        val ipAddress = IPv4Address(ip).toString
        val id = quill.run {
          quote {
            query[Node]
              .insert(
                _.name -> lift(
                  s"${hashId.encode(System.currentTimeMillis()).take(3)}_$ipAddress"
                ),
                _.publicKey -> lift(publicKey),
                _.networkId -> lift(network.id),
                _.setting -> lift(NodeSetting()),
                _.ip -> lift(ipAddress),
                _.nodeType -> lift(NodeType.Client),
                _.status -> lift(NodeStatus.Connected),
              )
              .returning(_.id)
          }
        }
        logger.info(
          s"new client:$id(${publicKey}) join network ${network.id}"
        )
        None
      case None =>
        Some("Network has no available IP")
    }
  }

  override def getSSOLoginInfo(
    request: SSOLoginInfoRequest
  ): Future[SSOLoginInfoResponse] = {
    // TODO: check NetworkId
    if (config.hasPath("auth.keycloak")) {
      Future.successful(
        SSOLoginInfoResponse(
          ssoUrl = config.get[String]("auth.keycloak.authServerUrl"),
          realm = config.get[String]("auth.keycloak.realm"),
          clientId = config.get[String]("auth.keycloak.frontClientId"),
        )
      )
    } else {
      Future.failed(
        io.grpc.Status.ABORTED
          .withDescription("DON'T SUPPORT SSO")
          .asException()
      )
    }
  }
}
