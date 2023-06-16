package very.util.keycloak

import org.scalatra.auth.ScentrySupport
import org.scalatra.auth.strategy.BasicAuthSupport
import com.typesafe.scalalogging.LazyLogging
import org.keycloak.TokenVerifier
import com.typesafe.scalalogging.Logger
import very.util.web.auth.AuthStrategy

import scala.util.{ Success, Failure }

class KeycloakJWTAuthStrategy(jwkTokenVerifier: JWKTokenVerifier, adminRole: Option[String], clientRole: Option[String])
  extends AuthStrategy[String] with LazyLogging {
  // JWT
  def name: String = KeycloakJWTAuthStrategy.name

  def adminAuth(token: String): Option[String] = {
    jwkTokenVerifier.verify(token) match {
      case Success(accessToken) =>
        if (adminRole.fold(true)(role => accessToken.getRealmAccess.getRoles.contains(role))) {
          Some(accessToken.getSubject)
        } else {
          logger.info(
            s"the user:${accessToken.getSubject} could not pass admin auth"
          )
          None
        }
      case Failure(exception) =>
        logger.debug(s"bad token:$token", exception)
        None
    }
  }

  def clientAuth(token: String): Option[String] = {
    jwkTokenVerifier.verify(token) match {
      case Success(accessToken) =>
        if (clientRole.fold(true)(role => accessToken.getRealmAccess.getRoles.contains(role))) {
          Some(accessToken.getSubject)
        } else {
          logger.info(
            s"the user:${accessToken.getSubject} could not pass client auth"
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
  val name: String = "Bearer"
}
