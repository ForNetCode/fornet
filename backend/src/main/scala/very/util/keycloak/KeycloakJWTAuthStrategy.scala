package very.util.keycloak

import org.scalatra.auth.ScentrySupport
import org.scalatra.auth.strategy.BasicAuthSupport
import com.typesafe.scalalogging.LazyLogging
import org.keycloak.TokenVerifier
import com.typesafe.scalalogging.Logger
import very.util.web.auth.AuthStrategy

import scala.util.{ Success, Failure }

class KeycloakJWTAuthStrategy(jwkTokenVerifier: JWKTokenVerifier, role: String)
  extends AuthStrategy[String] {
  def logger: Logger = com.typesafe.scalalogging.Logger(getClass.getName)

  // JWT
  def name: String = KeycloakJWTAuthStrategy.name

  def auth(token: String): Option[String] = {
    jwkTokenVerifier.verify(token) match {
      case Success(accessToken) =>
        if (accessToken.getRealmAccess.getRoles.contains(role)) {
          Some(accessToken.getSubject)
        } else {
          logger.info(
            s"the user:${accessToken.getSubject} does not have role:$role"
          )
          None
        }
      case Failure(exception) =>
        logger.debug(s"bad token:$token", exception)
        None
    }
  }

}

object KeycloakJWTAuthStrategy {
  val name:String = "Bearer"
}
