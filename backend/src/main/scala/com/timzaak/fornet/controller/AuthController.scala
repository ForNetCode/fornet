package com.timzaak.fornet.controller

import com.google.common.base.Charsets
import com.timzaak.fornet.config.AppConfig
import com.timzaak.fornet.controller.auth.AppAuthSupport
import com.timzaak.fornet.dao.{NetworkDao, NetworkStatus}
import com.timzaak.fornet.di.DI.hashId
import com.typesafe.config.Config
import org.hashids.Hashids
import org.scalatra.BadRequest.apply
import org.scalatra.json.JsonResult.apply
import org.scalatra.{ BadRequest, Ok }
import very.util.config.get
import very.util.web.Controller
import very.util.security.ID.toIntID
import zio.json.{ DeriveJsonDecoder, JsonDecoder }

import java.net.URLEncoder
import java.nio.charset.Charset
import java.util.Base64

case class SimpleTokenCheckReq(token: String)
given JsonDecoder[SimpleTokenCheckReq] = DeriveJsonDecoder.gen

trait AuthController(networkDao: NetworkDao, appConfig: AppConfig)(using config: Config, hashId: Hashids)
  extends Controller
  with AppAuthSupport {

  jPost("/st/check") { (req: SimpleTokenCheckReq) =>
    if (config.hasPath("auth.simple")) {
      if (config.get[String]("auth.simple.token") == req.token) {
        Ok()
      } else {
        logger.info(s"st checked")
        BadRequest("Invalid Token")
      }
    } else {
      BadRequest("Simple Token Auth closed!")
    }
  }

  // keycloak device code auth, needs keycloak browser login
  get("/oauth/:network_id/device_code") {
    val groupId = auth
    val networkId = params("network_id").toIntID
    if (appConfig.enableSAAS) {
      badResponse("SAAS do not support keycloak auth in command line")
    } else {
      networkDao
        .findById(networkId)
        .filter(n => n.status == NetworkStatus.Normal && n.groupId == groupId)
        .map { network =>
          val nId =
            URLEncoder.encode(networkId.secretId, Charsets.UTF_8)
          String(
            Base64.getEncoder.encode(
              s"2|${config.getString("server.grpc.endpoint")}|${network.tokenId.secretId}|$nId"
                .getBytes()
            )
          )
        }
    }
  }
}
