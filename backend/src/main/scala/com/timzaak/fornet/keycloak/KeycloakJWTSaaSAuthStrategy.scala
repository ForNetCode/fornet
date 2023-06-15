package com.timzaak.fornet.keycloak

import org.scalatra.auth.ScentrySupport
import org.scalatra.auth.strategy.BasicAuthSupport
import com.typesafe.scalalogging.LazyLogging
import org.keycloak.TokenVerifier
import com.typesafe.scalalogging.Logger
import very.util.keycloak.{ JWKTokenVerifier, KeycloakJWTAuthStrategy }
import very.util.web.auth.AuthStrategy

import scala.util.{ Failure, Success }

class KeycloakJWTSaaSAuthStrategy(
  jwkTokenVerifier: JWKTokenVerifier,
) extends AuthStrategy[String]
  with LazyLogging {

  def name: String = KeycloakJWTAuthStrategy.name

  def adminAuth(token: String): Option[String] = {
    jwkTokenVerifier.verify(token) match {
      case Success(accessToken) =>
        Some(accessToken.getSubject)
      case Failure(exception) =>
        logger.debug(s"bad token:$token", exception)
        None
    }
  }

  // SaaS do not support Client SSO Login
  def clientAuth(token: String): Option[String] = {
    None
  }

}
