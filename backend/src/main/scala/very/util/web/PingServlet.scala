package very.util.web

import org.scalatra.ScalatraServlet

class PingServlet extends ScalatraServlet {
  get("/") {
    "pong"
  }
}
