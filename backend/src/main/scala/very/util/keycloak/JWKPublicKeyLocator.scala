package very.util.keycloak

import org.keycloak.jose.jwk.{JSONWebKeySet, JWK}
import org.keycloak.util.{JWKSUtils, JsonSerialization}

import scala.jdk.CollectionConverters.*
import scala.util.Try
import java.security.PublicKey

class JWKPublicKeyLocator private(currentKeys:Map[String,PublicKey]) {
  
  def getPublicKey(kid:String) = {
    currentKeys.get(kid)
  }
}

object JWKPublicKeyLocator {
  def init(keycloakBaseUri: String, realm: String):Try[JWKPublicKeyLocator] = {
    Try {
      val data = scala.io.Source.fromURL(s"$keycloakBaseUri/realms/${realm}/protocol/openid-connect/certs").mkString
      val jwks = JsonSerialization.readValue(data, classOf[JSONWebKeySet])
      val publicKeys = JWKSUtils.getKeysForUse(jwks, JWK.Use.SIG)
      JWKPublicKeyLocator(publicKeys.asScala.toMap)
    }
  }
}
