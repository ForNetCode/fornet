package com.timzaak.fornet.controller

import com.timzaak.fornet.config.AppConfig
import com.typesafe.config.Config
import very.util.config.get
import very.util.web.Controller
import zio.json.ast.Json

trait AppInfoController(appConfig: AppConfig)(
  using config: Config,
) extends Controller {

  jGet("/") {
    if (config.hasPath("auth.keycloak")) {
      Json(
        "type" -> Json.Str("Bearer"),
        "url" -> Json.Str(config.get[String]("auth.keycloak.authServerUrl")),
        "realm" -> Json.Str(config.get[String]("auth.keycloak.realm")),
        "clientId" -> Json.Str(config.get[String]("auth.keycloak.frontClientId")),
        "saas" -> Json.Bool(appConfig.enableSAAS), // saas
      )
    } else {
      Json("type" -> Json.Str("ST"))
    }
  }
}
