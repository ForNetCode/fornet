package very.util.web.auth

import org.scalatra.{ ScalatraBase, Initializable }

import scala.util.Try

trait AuthSupport[User](using authStrategyProvider: AuthStrategyProvider[User]) {
  self: ScalatraBase =>

  def auth: User = {
    Option(request.getHeader("Authorization")) match {
      case Some(authorization) =>
        Try {
          val Array(strategy, token) = authorization.split(' ')
          authStrategyProvider.getStrategy(strategy).flatMap { authStrategy =>
            authStrategy.adminAuth(token)
          }
        }.toOption.flatten match {
          case Some(v) => v
          case None    => halt(org.scalatra.Unauthorized("bad token"))
        }
      case None => halt(org.scalatra.Unauthorized("no authorization header"))
    }

  }
}
