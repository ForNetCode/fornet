package com.timzaak.fornet.grpc

import ch.qos.logback.core.joran.action.Action
import com.google.common.base.Charsets
import com.google.protobuf.empty.Empty
import com.timzaak.fornet.config.AppConfig
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
import very.util.security.{IntID, TokenID}
import very.util.web.LogSupport
import zio.json.*
import zio.json.ast.{Json, JsonCursor}

import java.net.http.HttpRequest.BodyPublishers
import java.net.http.{HttpClient, HttpRequest}
import java.net.{URI, URLEncoder}
import java.time.{LocalDateTime, OffsetDateTime}
import scala.concurrent.Future
import scala.util.{Failure, Success, Try}

class AuthGRPCController(
  nodeDao: NodeDao,
  networkDao: NetworkDao,
  deviceDao: DeviceDao,
  nodeChangeNotifyService: NodeChangeNotifyService,
  config: Config,
  nodeAuthService: NodeAuthService,
  authStrategyProvider: AppAuthStrategyProvider,
  appConfig: AppConfig,
)(using quill: DB, hashId: Hashids)
  extends AuthGrpc.Auth
  with LogSupport {

  import very.util.config.get
  private val mqttClientUrl = config.get[String]("mqtt.clientUrl")

  import quill.{*, given}
  private def errorResponse(message: String) = ActionResponse(ActionResponse.Response.Error(message))
  private def successResponse(secretId:String) = ActionResponse(
    ActionResponse.Response.Success(
      com.timzaak.fornet.protobuf.auth.SuccessResponse(mqttClientUrl, secretId)
    )
  )

  private def getDevice(deviceId:Option[String], publicKey:String):Try[Device]  = {
    deviceId.map { deviceIdStr =>
      Try(TokenID(deviceIdStr)).flatMap { deviceTokenId =>
        deviceDao.findByTokenID(deviceTokenId).map(Success(_)).getOrElse(Failure(new Exception("device TokenId invalid")))
      }
    }.getOrElse {
      // publicKey always be unique!
      val token = TokenID.randomToken()
      val device = quill.run(query[Device].insert(
        _.token -> lift(token),
        _.publicKey -> lift(publicKey),
      ).onConflictIgnore(_.publicKey).returning(v => v))
      Success(device)
    }
  }
  override def inviteConfirm(
    request: InviteConfirmRequest
  ): Future[ActionResponse] = {
    var params = Seq(request.networkId)
    request.deviceId.foreach(deviceTokenId => params = params.appended(deviceTokenId))
    if (request.nodeId.nonEmpty) {
      params = params.appended(request.nodeId.get)
    }

    if (request.encrypt.exists(v => nodeAuthService.validate(v, params))) {
      val networkId = IntID(request.networkId)
      val publicKey = request.encrypt.get.publicKey

      val response = (request.nodeId, getDevice(request.deviceId, request.encrypt.get.publicKey)) match {
        case (_, Failure(e)) => errorResponse("Illegal Arguments")
        case (Some(nodeIdStr), Success(device)) =>
          val nodeId = IntID(nodeIdStr)
          val changeCount = quill.run {
            quote {
              query[Node]
                .filter(n =>
                  n.networkId == lift(networkId) && n.id == lift(
                    nodeId
                  ) && n.status == lift(NodeStatus.Waiting)
                )
                .update(
                  _.deviceId -> lift(device.id),
                  _.status -> lift(NodeStatus.Normal),
                  _.updatedAt -> lift(OffsetDateTime.now()),
                )
            }
          }
          if (changeCount > 0) {
            // notify relay node online
            val node = nodeDao.findById(networkId, nodeId).get
            nodeChangeNotifyService.nodeStatusChangeNotify(
              node,
              device,
              NodeStatus.Waiting,
              NodeStatus.Normal
            )
            successResponse(node.id.secretId)
          } else {
            errorResponse("already active or error response")
          }
        case (None, Success(device))=>
          createNode(networkId, publicKey, device) match {
            case Left(value) => errorResponse(value)
            case Right(id) =>
              successResponse(id.secretId)
          }
      }
      Future.successful(response)
    } else {
      Future.successful(
        errorResponse("Illegal Arguments")
      )
    }
  }

  import very.util.config.get

  override def oauthDeviceCodeConfirm(
    request: OAuthDeviceCodeRequest
  ): Future[ActionResponse] = {
    var params = Seq(request.accessToken, request.deviceCode)
    request.deviceId.foreach(deviceId => params = params.appended(deviceId))
    params = params.appended(request.networkId)

    if (!appConfig.enableSAAS && request.encrypt.exists(v => nodeAuthService.validate(v, params))) {
      if (config.hasPath("auth.keycloak")) {
        val authResult = authStrategyProvider
          .getStrategy(KeycloakJWTAuthStrategy.name)
          .flatMap { auth => auth.clientAuth(request.accessToken) }
          .toRight("auth process error, please check server error log")

        authResult match {
          case Left(value) =>
            Future.successful(errorResponse(value))
          case Right(userId) =>
            val publicKey = request.encrypt.get.publicKey
            val networkId = IntID(request.networkId)

            logger.info(
              s"user:${userId},networkId:${request.networkId}, publicKey:${request.encrypt.get.publicKey} register device with code:${request.deviceCode}"
            )
            val response = getDevice(request.deviceId, request.encrypt.get.publicKey) match {
              case Success(device) =>
                createNode(networkId, publicKey, device) match {
                  case Left(value) => errorResponse(value)
                  case Right(id) => successResponse(id.secretId)
                }
              case Failure(_) => errorResponse("Illegal Arguments")
            }
            Future.successful(response)
        }
      } else {
        Future.successful(
          errorResponse("do not support keycloak now")
        )
      }
    } else {
      Future.successful(
        errorResponse("Illegal Arguments")
      )
    }
  }

  //TODO: create node trigger state change push?
  private def createNode(networkId: IntID, publicKey: String, device: Device):Either[String, IntID] = {
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
                _.deviceId -> lift(device.id),
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
          s"new client:${id.id}(${publicKey}) join network ${network.id}"
        )
        Right(id)
      case None =>
        Left("Network has no available IP")
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
