package com.timzaak.fornet.dao

// import io.getquill.{UpdateMeta, updateMeta}

import com.timzaak.fornet.dao.NetworkProtocol.TCP
import org.hashids.Hashids
import very.util.persistence.quill.DBSerializer
import very.util.security.{IntID, TokenID}
import zio.json.*

import java.time.OffsetDateTime
import scala.util.{Failure, Success, Try}

enum NetworkStatus {
  case Normal, Delete
}
object NetworkStatus {
  given JsonEncoder[NetworkStatus] = JsonEncoder[Int].contramap(_.ordinal)

  given JsonDecoder[NetworkStatus] = JsonDecoder[Int].mapOrFail { e =>
    Try(NetworkStatus.fromOrdinal(e)) match {
      case Success(v) => Right(v)
      case Failure(_) => Left("no matching NodeType enum value")
    }
  }
}

enum NetworkProtocol {
  case TCP, UDP

  import com.timzaak.fornet.protobuf.config.Protocol as PProtocol
  def gRPCProtocol: PProtocol = {
    this match {
      case TCP => PProtocol.Protocol_TCP
      case UDP => PProtocol.Protocol_UDP
    }
  }

  given JsonEncoder[NetworkProtocol] = JsonEncoder[Int].contramap(_.ordinal)

  given JsonDecoder[NetworkProtocol] = JsonDecoder[Int].mapOrFail { e =>
    Try(NetworkProtocol.fromOrdinal(e)) match {
      case Success(v) => Right(v)
      case Failure(_) => Left("no matching NodeType enum value")
    }
  }
}
object NetworkProtocol {
  given JsonEncoder[NetworkProtocol] = JsonEncoder[Int].contramap(_.ordinal)

  given JsonDecoder[NetworkProtocol] = JsonDecoder[Int].mapOrFail { e =>
    Try(NetworkProtocol.fromOrdinal(e)) match {
      case Success(v) => Right(v)
      case Failure(_) => Left("no matching NetworkProtocol enum value")
    }
  }
}

case class Network(
  id: IntID,
  name: String,
  token: String,
  groupId: String,
  addressRange: String,
  setting: NetworkSetting,
  status: NetworkStatus,
  createdAt: OffsetDateTime,
  updatedAt: OffsetDateTime,
) {
  def tokenId:TokenID = TokenID(id, token)
}
//object Network {
//  given networkUpdateMeta:UpdateMeta[Network] = updateMeta[Network](_.id)
//}
case class NetworkSetting(
  port: Int = 51820,
  keepAlive: Int = 30,
  mtu: Int = 1420,
  protocol: NetworkProtocol = NetworkProtocol.UDP,
  dns: Option[Seq[String]] = None,
) extends DBSerializer

object Network {
  import very.util.web.json.{intIDDecoder, intIDEncoder}
  given networkDerive(using hashId: Hashids): JsonCodec[Network] = DeriveJsonCodec.gen
}
object NetworkSetting {
  given JsonCodec[NetworkSetting] = DeriveJsonCodec.gen
}

import io.getquill.*
import org.hashids.Hashids

class NetworkDao(using quill: DB, hashIds: Hashids) {
  import quill.{*, given}

  def findById(id: IntID): Option[Network] = {
    quill.run(quote(query[Network]).filter(_.id == lift(id)).single).headOption
  }
  def findByIds(ids: List[IntID]): List[Network] = {
    quill.run(quote(query[Network]).filter(v => lift(ids).contains(v.id)))
  }

  def countByGroupId(groupId: String): Long = {
    quill.run(quote(query[Network]).filter(_.groupId == lift(groupId)).size)
  }

  def existGroupNetwork(networkId: IntID, groupId: String): Boolean = {
    quill.run(quote(query[Network]).filter(n => n.id == lift(networkId) && n.groupId == lift(groupId)).nonEmpty)
  }

  def findByTokenId(tokenId:TokenID): Option[Network] = 
    quill.run(quote(query[Network]).filter(n => n.id == lift(tokenId.intId) && n.name == lift(tokenId.token)).single).headOption
}
