package com.timzaak.fornet.controller

import com.timzaak.fornet.controller.auth.AppAuthSupport
import com.timzaak.fornet.dao.*
import com.timzaak.fornet.grpc.convert.EntityConvert
import com.timzaak.fornet.pubsub.NodeChangeNotifyService
import com.typesafe.config.Config
import inet.ipaddr.IPAddress.IPVersion
import inet.ipaddr.IPAddressString
import inet.ipaddr.ipv4.IPv4Address
import io.getquill.*
import org.hashids.Hashids
import org.scalatra.Accepted
import very.util.web.Controller
import very.util.web.json.JsonResponse
import zio.json.*
import zio.prelude.Validation

import java.time.OffsetDateTime
import java.util.Base64

case class CreateNodeReq(
  name: String,
  ip: Option[String],
  setting: NodeSetting,
  nodeType: NodeType,
)

given JsonDecoder[CreateNodeReq] = DeriveJsonDecoder.gen

case class UpdateNodeInfoReq(
  name: String,
  setting: NodeSetting,
)

given JsonDecoder[UpdateNodeInfoReq] = DeriveJsonDecoder.gen

case class UpdateNodeStatusReq(
  status: NodeStatus
) {
  // assert(NodeStatus.Waiting != status)
}
given JsonDecoder[UpdateNodeStatusReq] = DeriveJsonDecoder.gen

trait NodeController(
  nodeDao: NodeDao,
  networkDao: NetworkDao,
  nodeChangeNotifyService: NodeChangeNotifyService,
)(using quill: DB, hashId: Hashids, config: Config)
  extends Controller
  with AppAuthSupport {
  import quill.{*, given}

  private def _networkId: Int = params("networkId").toInt
  private def _nodeId: Int = params("nodeId").toInt

  def checkAuth = {
    val groupId = auth
    val networkId = _networkId
    if(!networkDao.existGroupNetwork(networkId, groupId)) {
      halt(org.scalatra.MethodNotAllowed("bad request"))
    }

  }

  jGet("/:networkId") {
    checkAuth
    val result = quill.run {
      quote {
        query[Node]
          .filter(_.networkId == lift(_networkId))
          .sortBy(_.id)(Ord.desc)
          .page
      }
    }
    result
  }

  jGet("/:networkId/:nodeId") {
    checkAuth
    val data = quill.run {
      quote(
        query[Node]
          .filter(n => n.id == lift(_nodeId) && n.networkId == lift(_networkId))
          .single
      )
    }.headOption
    data
  }

  jPut("/:networkId/:nodeId") { (req: UpdateNodeInfoReq) =>
    checkAuth
    val nodeId = _nodeId
    val networkId = _networkId
    val oldNode = nodeDao.findById(networkId, nodeId).get

    quill.run {
      quote {
        query[Node]
          .filter(n => n.id == lift(nodeId) && n.networkId == lift(networkId))
          .update(
            _.name -> lift(req.name),
            _.setting -> lift(req.setting),
            _.updatedAt -> lift(OffsetDateTime.now()),
          )
      }
    }

    if (oldNode.setting != req.setting && oldNode.status == NodeStatus.Normal) {
      // notify self change
      nodeChangeNotifyService.nodeInfoChangeNotify(oldNode, req.setting)
    }
    Accepted()
  }

  jPut("/:networkId/:nodeId/status") { (req: UpdateNodeStatusReq) =>
    checkAuth
    val networkId = _networkId
    val nodeId = _nodeId
    val oldNode = nodeDao.findById(networkId, nodeId).get
    val changeNumber = quill.run {
      quote {
        query[Node]
          .filter(n =>
            n.id == lift(nodeId) && n.networkId == lift(
              networkId
            ) && n.status != lift(req.status)
              && n.status != lift(NodeStatus.Delete)
          )
          .update(
            _.updatedAt -> lift(OffsetDateTime.now()),
            _.status -> lift(req.status),
          )
      }
    }
    if (changeNumber > 0) {
      nodeChangeNotifyService.nodeStatusChangeNotify(
        oldNode,
        oldNode.status,
        req.status
      )
    }
    Accepted()
  }

  get("/:networkId/:nodeId/active_code") {
    nodeDao
      .findById(_networkId, _nodeId)
      .filter(_.status == NodeStatus.Waiting)
      .map { _ =>
        String(
          Base64.getEncoder.encode(
            s"1|${config.getString("server.grpc.endpoint")}|${hashId.encode(
                params("networkId").toLong
              )}|${hashId.encode(params("nodeId").toLong)}".getBytes
          )
        )
      }
  }

  jPost("/:networkId") { (req: CreateNodeReq) =>
    checkAuth
    val networkId = _networkId
    val ipValidation = networkDao.findById(networkId) match {
      case Some(network) =>
        val usedIp = nodeDao
          .getUsedIps(networkId)
          .map(ip =>
            IPAddressString(ip)
              .toAddress(IPVersion.IPV4)
              .asInstanceOf[IPv4Address]
              .intValue()
          )
          .toSet
        val addressRange = IPAddressString(network.addressRange)
          .toAddress(IPVersion.IPV4)
          .asInstanceOf[IPv4Address]
        req.ip.map { ip =>
          req.nodeType match {
            case NodeType.Relay  => ipV4Range(ip)
            case NodeType.Client => privateIpValid(ip)
          }
        } match {
          case Some(v: Validation.Success[String, IPv4Address]) =>
            if (
              usedIp
                .contains(v.value.intValue()) || !addressRange.prefixContains(
                v.value
              )
            ) {
              Validation.fail(
                i18n("error.ipv4_address_in_range")(
                  v.value,
                  addressRange.toString
                )
              )
            } else {
              v
            }
          case Some(e: Validation.Failure[_, _]) => e
          case None                              =>
            // generate ip
            val addressRange = IPAddressString(network.addressRange)
              .toAddress(IPVersion.IPV4)
              .toPrefixBlock()
              .asInstanceOf[IPv4Address]
            (addressRange.getLowerNonZeroHost
              .intValue() to addressRange.getUpper.intValue())
              .find(!usedIp.contains(_)) match {
              case Some(ip) =>
                req.nodeType match {
                  case NodeType.Client => Validation.succeed(IPv4Address(ip))
                  case NodeType.Relay =>
                    Validation.succeed(
                      IPv4Address(ip, addressRange.getNetworkPrefixLength)
                    )
                }
              case None =>
                Validation.fail(i18n("error.ip_no_free_ip_in_network")())
            }
        }
      case None =>
        Validation.fail(i18n("error.parameter_error")())
    }

    for { ip <- ipValidation } yield {
      val id = quill.run {
        quote {
          query[Node]
            .insert(
              _.name -> lift(req.name),
              _.networkId -> lift(networkId),
              _.setting -> lift(req.setting),
              _.ip -> lift(ip.toString),
              _.nodeType -> lift(req.nodeType),
              _.status -> lift(NodeStatus.Waiting),
            )
            .returning(_.id)
        }
      }
      created(id)
    }
  }
}
