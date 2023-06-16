package com.timzaak.fornet

import org.scalatra.LifeCycle
import jakarta.servlet.ServletContext
import org.json4s.Formats
import very.util.web.PingServlet
import com.timzaak.fornet.controller.*
import com.timzaak.fornet.di.DI

import scala.annotation.unused

@unused("used by reflect")
class ScalatraBootstrap extends LifeCycle {
  override def init(context: ServletContext): Unit = {
    // Disables cookies, but required because browsers will not allow passing credentials to wildcard domains
    // context.setInitParameter("org.scalatra.cors.allowCredentials", "false")

    context mount (DI.networkController, "/api/network")
    context mount (DI.nodeController, "/api/node")
    context mount (DI.authController, "/api/auth")
    context mount (DI.appInfoController, "/api/info")
    context mount (DI.mqttCallbackController, "/mqtt")
    context mount (PingServlet(), "/ping")
  }

  override def destroy(context: ServletContext): Unit = {
    super.destroy(context)
  }
}
