package com.timzaak.fornet.dao

// import io.getquill.{UpdateMeta, updateMeta}

import very.util.persistence.quill.DBSerializer
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
  id: Int,
  name: String,
  addressRange: String,
  setting: NetworkSetting,
  status: NetworkStatus,
  createdAt: OffsetDateTime,
  updatedAt: OffsetDateTime
)
//object Network {
//  given networkUpdateMeta:UpdateMeta[Network] = updateMeta[Network](_.id)
//}
case class NetworkSetting(
  port: Int = 51820,
  keepAlive: Int = 30,
  mtu: Int = 1420,
  protocol:NetworkProtocol = NetworkProtocol.UDP,
  dns: Option[Seq[String]] = None,
) extends DBSerializer

object Network {
  given JsonCodec[Network] = DeriveJsonCodec.gen
}
object NetworkSetting {
  given JsonCodec[NetworkSetting] = DeriveJsonCodec.gen
}

import io.getquill.*
class NetworkDao(using quill: DB) {
  import quill.{*, given}

  def findById(id: Int): Option[Network] = {
    quill.run(quote(query[Network]).filter(_.id == lift(id)).single).headOption
  }
  def findByIds(ids: List[Int]): List[Network] = {
    quill.run(quote(query[Network]).filter(v => lift(ids).contains(v.id)))
  }
}
