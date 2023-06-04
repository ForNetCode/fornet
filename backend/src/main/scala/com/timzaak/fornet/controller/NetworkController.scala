package com.timzaak.fornet.controller

import com.google.common.net.InetAddresses
import com.timzaak.fornet.config.AppConfig
import com.timzaak.fornet.controller.auth.AppAuthSupport
import com.timzaak.fornet.dao.{ DB, Network, NetworkDao, NetworkSetting }
import com.typesafe.config.Config
import org.hashids.Hashids
import very.util.security.IntID

import java.util.Base64
//import org.json4s.Formats
import com.timzaak.fornet.dao.NetworkStatus
import io.getquill.*
import org.scalatra.i18n.I18nSupport
import org.scalatra.json.*
import org.scalatra.*
import very.util.web.Controller
import very.util.web.validate.ValidationExtra
import very.util.security.IntID.toIntID
import zio.json.{ DeriveJsonDecoder, JsonDecoder }

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
  appConfig: AppConfig,
)(using quill: DB, config: Config, hashId: Hashids)
  extends Controller
  with AppAuthSupport {

//  import org.json4s.jvalue2extractable
  import quill.{ *, given }

  def _networkId: IntID = params("id").toIntID

  jGet[Network]("/") {
    val groupId = auth
    params.get("name") match {
      case Some(name) if name.nonEmpty =>
        pageWithCount(
          query[Network]
            .filter(v => sql"${v.name} like ${lift(s"%${name}%")}".asCondition)
            .filter(_.status == lift(NetworkStatus.Normal))
            .filter(_.groupId == lift(groupId))
        )(_.sortBy(_.id)(Ord.desc))
      case _ =>
        pageWithCount(
          query[Network].filter(_.status == lift(NetworkStatus.Normal))
        )(_.sortBy(_.id)(Ord.desc))
    }
  }

  jPost("/") { (req: CreateNetworkReq) =>
    val groupId = auth
    import zio.*

    for {
      _ <- ipV4Range(req.addressRange)
    } yield {
      if (appConfig.enableSAAS && networkDao.countByGroupId(groupId) > 10) {
        badResponse("network number is limited to 10")
      } else {
        val id = quill.run {
          quote {
            query[Network]
              .insert(
                _.name -> lift(req.name),
                _.addressRange -> lift(req.addressRange),
                _.setting -> lift(NetworkSetting()),
                _.groupId -> lift(groupId),
              )
              .returning(_.id)
          }
        }
        created(id)
      }
    }
  }

  jGet("/:id") {
    val groupId = auth
    networkDao.findById(_networkId).filter(n => n.status == NetworkStatus.Normal && n.groupId == groupId)
  }

  get("/:id/invite_code") {
    val groupId = auth
    val networkId = _networkId
    networkDao
      .findById(networkId)
      .filter(n => n.status == NetworkStatus.Normal && n.groupId == groupId)
      .map { _ =>
        String(
          Base64.getEncoder.encode(
            s"1|${config.getString("server.grpc.endpoint")}|${networkId.secretId}"
              .getBytes()
          )
        )
      }
  }

  jPut("/:id") { (data: UpdateNetworkReq) =>
    val groupId = auth
    val id = params("id").toIntID
    for {
      _ <- ipV4Range(data.addressRange)
    } yield {
      quill.run {
        quote {
          query[Network]
            .filter(n => n.id == lift(id) && n.groupId == lift(groupId))
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
    val groupId = auth
    val networkId = _networkId
    val changeCount = quill.run {
      quote {
        query[Network]
          .filter(n => n.id == lift(networkId) && n.status == lift(NetworkStatus.Normal) && n.groupId == lift(groupId))
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
