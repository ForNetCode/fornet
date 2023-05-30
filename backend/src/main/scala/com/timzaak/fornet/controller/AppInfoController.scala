package com.timzaak.fornet.controller

import com.timzaak.fornet.config.AppConfig
import com.typesafe.config.Config
import very.util.config.get
import very.util.web.Controller

trait AppInfoController(appConfig: AppConfig)(
  using config: Config,
) extends Controller {

  jGet("/") {
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
}
