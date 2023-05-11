package com.timzaak.fornet.controller

import com.google.common.base.Charsets
import com.timzaak.fornet.controller.auth.AppAuthSupport
import com.timzaak.fornet.dao.{NetworkDao, NetworkStatus}
import com.timzaak.fornet.di.DI.hashId
import com.typesafe.config.Config
import org.hashids.Hashids
import org.scalatra.BadRequest.apply
import org.scalatra.json.JsonResult.apply
import org.scalatra.{BadRequest, Ok}
import very.util.config.get
import very.util.web.Controller
import zio.json.{DeriveJsonDecoder, JsonDecoder}

import java.net.URLEncoder
import java.nio.charset.Charset
import java.util.Base64

case class SimpleTokenCheckReq(token: String)
given JsonDecoder[SimpleTokenCheckReq] = DeriveJsonDecoder.gen

trait AuthController(networkDao: NetworkDao)(using
  config: Config,
  hashId: Hashids
) extends Controller
  with AppAuthSupport {

  jGet("/type") {
    if (config.hasPath("auth.keycloak")) {
      Map(
        "type" -> "Bearer",
        "url" -> config.get[String]("auth.keycloak.authServerUrl"),
        "realm" -> config.get[String]("auth.keycloak.realm"),
        "clientId" -> config.get[String]("auth.keycloak.frontClientId"),
      )
    } else {
      Map("type" -> "ST")
    }
  }

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

  get("/oauth/:network_id/device_code") {
    auth
    val networkId = params("network_id").toInt
    networkDao
      .findById(networkId)
      .filter(_.status == NetworkStatus.Normal)
      .map { _ =>
        val nId =
          URLEncoder.encode(hashId.encode(networkId.toLong), Charsets.UTF_8)
        String(
          Base64.getEncoder.encode(
            s"2|${config.getString("server.grpc.endpoint")}|${nId}"
              .getBytes()
          )
        )
      }
  }

  /*
  jGet("/oauth/device_code") {
    //val nId = params("n_id")
    //val networkId = hashId.decode(nId).head.toInt
    Map(
      "grpc_url" -> config.get[String]("server.grpc.endpoint"),
      "sso_url" -> config.get[String]("auth.keycloak.authServerUrl"),
      "realm" -> config.get[String]("auth.keycloak.realm"),
      "client_id" -> config.get[String]("auth.keycloak.frontClientId"),
    )
  }
   */
}
