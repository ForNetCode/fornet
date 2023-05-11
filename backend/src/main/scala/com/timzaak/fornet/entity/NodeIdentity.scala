package com.timzaak.fornet.entity

import org.bouncycastle.crypto.params.Ed25519PublicKeyParameters
import org.bouncycastle.crypto.signers.Ed25519Signer
import zio.json.JsonCodec

import java.util.Base64
import java.nio.charset.StandardCharsets

case class NodeIdentity(networkId: Int, nodeId: Int)

case class PublicKey(key: String) extends Key {
  def validate(data: String, signature: String): Boolean = {
    val publicKey = Ed25519PublicKeyParameters(ed25519)
    val verifier = Ed25519Signer()
    val m = data.getBytes(StandardCharsets.UTF_8)
    verifier.init(false, publicKey)
    verifier.update(m, 0, m.length)
    verifier.verifySignature(Base64.getDecoder.decode(signature))
  }
}

object PublicKey {
  given JsonCodec[PublicKey] = JsonCodec[String].transform(PublicKey(_), _.key)
}

case class PrivateKey(key: String) extends Key
trait Key {
  def key: String

  def ed25519: Array[Byte] = Base64.getDecoder.decode(key).drop(32)

  def x25519: Array[Byte] = Base64.getDecoder.decode(key).take(32)
}
