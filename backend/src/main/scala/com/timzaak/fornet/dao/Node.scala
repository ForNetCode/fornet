package com.timzaak.fornet.dao

import io.getquill.MappedEncoding
import very.util.persistence.quill.DBSerializer
import zio.json.*

import java.time.OffsetDateTime
import scala.util.{Failure, Success, Try}

enum NodeType {
  // Normal: Fornet Client

  case Client, Relay
}

object NodeType {

  given JsonEncoder[NodeType] = JsonEncoder[Int].contramap(_.ordinal)

  given JsonDecoder[NodeType] = JsonDecoder[Int].mapOrFail { e =>
    Try(NodeType.fromOrdinal(e)) match {
      case Success(v) => Right(v)
      case Failure(_) => Left("no matching NodeType enum value")
    }
  }

  // given JsonCodec[NodeType] = DeriveJsonCodec.gen

}

enum NodeStatus {
  // Connected is for client which has connected server without confirm
  // There has a transform for grpc NodeStatus
  case Waiting, Connected, Normal, Forbid, Delete

  import com.timzaak.fornet.protobuf.config.NodeStatus as PNodeStatus
  def gRPCNodeStatus: PNodeStatus = {

    this match {
      case NodeStatus.Waiting | NodeStatus.Connected => PNodeStatus.NODE_WAITING
      case NodeStatus.Normal                         => PNodeStatus.NODE_NORMAL
      case _                                         => PNodeStatus.NODE_FORBID
    }
  }
}

object NodeStatus {
  // given JsonCodec[NodeStatus] = DeriveJsonCodec.gen
  given JsonEncoder[NodeStatus] = JsonEncoder[Int].contramap(_.ordinal)

  given JsonDecoder[NodeStatus] = JsonDecoder[Int].mapOrFail { e =>
    Try(NodeStatus.fromOrdinal(e)) match {
      case Success(v) => Right(v)
      case Failure(_) => Left("no matching NodeType enum value")
    }
  }
}

case class Node(
  id: Int,
  name: String,
  networkId: Int,
  ip: String,
  publicKey: String,
  setting: NodeSetting,
  nodeType: NodeType,
  status: NodeStatus,
  createdAt: OffsetDateTime,
  updatedAt: OffsetDateTime,
) {
  def address(network: Network): String = {
    // TODO: support IPv6
    nodeType match {
      case NodeType.Relay  => ip
      case NodeType.Client => s"$ip/${network.addressRange.split('/').last}"
    }
  }
  
  def realStatus(networkStatus:NetworkStatus):NodeStatus = {
    if(networkStatus == NetworkStatus.Delete) {
      NodeStatus.Delete
    } else {
      status
    }
  }

  def peerAddress: String = {
    nodeType match {
      case NodeType.Relay  => ip
      case NodeType.Client => s"$ip/32"
    }
  }
}

object Node {
  given JsonCodec[Node] = DeriveJsonCodec.gen
}

case class NodeSetting(
  port: Option[Int] = None,
  keepAlive: Option[Int] = None,
  mtu: Option[Int] = None,
  endpoint: Option[String] = None,
  dns: Option[Seq[String]] = None,
  // iptables -A FORWARD -i fort0 -j ACCEPT; iptables -A FORWARD -o fort0 -j ACCEPT; iptables -t nat -A POSTROUTING -o eth0 -j MASQUERADE
  postUp: Option[String] = None,
  // iptables -D FORWARD -i fort0 -j ACCEPT; iptables -D FORWARD -o fort0 -j ACCEPT; iptables -t nat -D POSTROUTING -o eth0 -j MASQUERADE
  postDown: Option[String] = None,
) extends DBSerializer

object NodeSetting {
  given JsonCodec[NodeSetting] = DeriveJsonCodec.gen
}

import io.getquill.*

class NodeDao(using quill: DB) {

  import quill.{*, given}

  def findIdByPublicKey(publicKey: String, networkId: Int): Option[Int] =
    quill.run {
      quote {
        query[Node]
          .filter(n =>
            n.publicKey == lift(publicKey) && n.networkId == lift(networkId)
          )
          .map(_.id)
          .single
      }
    }.headOption

  def findById(networkId: Int, nodeId: Int): Option[Node] = quill.run {
    quote {
      query[Node]
        .filter(n => n.networkId == lift(networkId) && n.id == lift(nodeId))
        .single
    }
  }.headOption

  def findByPublicKey(publicKey: String): List[Node] = quill.run {
    quote {
      query[Node]
        .filter(_.publicKey == lift(publicKey))
    }
  }

  def getUsedIps(networkId: Int): Seq[String] = quill.run {
    quote {
      query[Node]
        .filter(n =>
          n.networkId == lift(networkId) && n.ip != lift(
            ""
          ) && n.status != lift(NodeStatus.Delete)
        )
        .map(_.ip)
    }
  }

  def getAllAvailableNodeIds(networkId: Int): Seq[Int] = quill.run {
    quote {
      query[Node]
        .filter(n =>
          n.networkId == lift(networkId) && n.status == lift(NodeStatus.Normal)
        )
        .map(_.id)
    }
  }

  def getAllAvailableNodes(networkId:Int):Seq[Node] = quill.run {
    quote {
      query[Node].filter(n => n.networkId == lift(networkId) && n.status == lift(NodeStatus.Normal))
    }
  }
  def getAllAvailableNodes(
    networkId: Int,
    exceptNodeId: Int,
    nodeType: NodeType
  ): Seq[Node] = quill.run {
    quote {
      query[Node].filter(n =>
        n.networkId == lift(networkId) && n.status == lift(
          NodeStatus.Normal
        ) && n.id != lift(exceptNodeId)
          && n.nodeType == lift(nodeType)
      )
    }
  }
}
