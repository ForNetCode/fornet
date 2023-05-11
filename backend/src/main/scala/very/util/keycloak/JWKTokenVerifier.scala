package very.util.keycloak

import org.keycloak.TokenVerifier
import org.keycloak.representations.AccessToken

import scala.util.Try

class JWKTokenVerifier(
  publicKeyLocator: JWKPublicKeyLocator,
  keycloakBaseUri: String,
  realm: String,
) {
  private val realmUrl = s"$keycloakBaseUri/realms/${realm}"
  def verify(token: String): Try[AccessToken] = {
    val tokenVerifier = TokenVerifier
      .create(token, classOf[AccessToken])
      .withDefaultChecks()
      .realmUrl(realmUrl)
    val kid = tokenVerifier.getHeader.getKeyId
    Try {
      tokenVerifier.publicKey(publicKeyLocator.getPublicKey(kid).get)
      tokenVerifier.verify()
      // subject is user_id
    }.map(_ => tokenVerifier.getToken)

  }
}
