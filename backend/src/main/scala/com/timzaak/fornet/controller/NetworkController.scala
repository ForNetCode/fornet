package com.timzaak.fornet.controller

import com.google.common.net.InetAddresses
import com.timzaak.fornet.controller.auth.AppAuthSupport
import com.timzaak.fornet.dao.{DB, Network, NetworkDao, NetworkSetting}
import com.typesafe.config.Config
import org.hashids.Hashids

import java.util.Base64
//import org.json4s.Formats
import com.timzaak.fornet.dao.NetworkStatus
import io.getquill.*
import org.scalatra.i18n.I18nSupport
import org.scalatra.json.*
import org.scalatra.*
import very.util.web.Controller
import very.util.web.validate.ValidationExtra
import zio.json.{DeriveJsonDecoder, JsonDecoder}

import java.time.OffsetDateTime

case class CreateNetworkReq(name: String, addressRange: String)
given JsonDecoder[CreateNetworkReq] = DeriveJsonDecoder.gen
case class UpdateNetworkReq(
  name: String,
  addressRange: String,
  setting: NetworkSetting,
)
given JsonDecoder[UpdateNetworkReq] = DeriveJsonDecoder.gen
trait NetworkController(
  networkDao: NetworkDao,
)(using quill: DB, config: Config, hashId: Hashids)
  extends Controller
  with AppAuthSupport {

//  import org.json4s.jvalue2extractable
  import quill.{*, given}

  def _networkId = params("id").toInt

  jGet[Network]("/") {
    auth
    params.get("name") match {
      case Some(name) if name.nonEmpty =>
        pageWithCount(
          query[Network]
            .filter(v => sql"${v.name} like ${lift(s"%${name}%")}".asCondition)
            .filter(_.status == lift(NetworkStatus.Normal))
        )(_.sortBy(_.id)(Ord.desc))
      case _ =>
        pageWithCount(
          query[Network].filter(_.status == lift(NetworkStatus.Normal))
        )(_.sortBy(_.id)(Ord.desc))
    }
  }

  jPost("/") { (req: CreateNetworkReq) =>
    auth
    import zio.*

    for {
      _ <- ipV4Range(req.addressRange)
    } yield {
      val id = quill.run {
        quote {
          query[Network]
            .insert(
              _.name -> lift(req.name),
              _.addressRange -> lift(req.addressRange),
              _.setting -> lift(NetworkSetting()),
            )
            .returning(_.id)
        }
      }
      created(id)
    }
  }

  jGet("/:id") {
    auth
    networkDao.findById(_networkId).filter(_.status == NetworkStatus.Normal)
  }

  get("/:id/invite_code") {
    auth
    val networkId = _networkId
    networkDao
      .findById(networkId)
      .filter(_.status == NetworkStatus.Normal)
      .map { _ =>
        String(
          Base64.getEncoder.encode(
            s"1|${config.getString("server.grpc.endpoint")}|${hashId.encode(networkId.toLong)}"
              .getBytes()
          )
        )
      }
  }

  jPut("/:id") { (data: UpdateNetworkReq) =>
    auth
    val id = params("id").toInt
    for {
      _ <- ipV4Range(data.addressRange)
    } yield {
      quill.run {
        quote {
          query[Network]
            .filter(_.id == lift(id))
            .update(
              _.name -> lift(data.name),
              _.addressRange -> lift(data.addressRange),
              _.setting -> lift(data.setting),
              _.updatedAt -> lift(OffsetDateTime.now()),
            )
        }
      }
      Accepted()
    }
  }

  delete("/:id") {
    auth
    val networkId = _networkId
    val changeCount = quill.run {
      quote {
        query[Network]
          .filter(n =>
            n.id == lift(networkId) && n.status == lift(NetworkStatus.Normal)
          )
          .update(
            _.status -> lift(NetworkStatus.Delete)
          )
      }
    }
    if (changeCount > 0) {
      // TODO:
      // kickoff all nodes in the network
      // change all node status to Deleted
    }
    Accepted()
  }
}
