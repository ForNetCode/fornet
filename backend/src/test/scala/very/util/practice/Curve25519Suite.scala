package very.util.practice

import munit.FunSuite
import org.bouncycastle.asn1.edec.EdECObjectIdentifiers
import org.bouncycastle.asn1.x509.SubjectPublicKeyInfo
import org.bouncycastle.crypto.generators.{
  Ed25519KeyPairGenerator,
  X25519KeyPairGenerator
}
import org.bouncycastle.crypto.params.{
  Ed25519KeyGenerationParameters,
  Ed25519PrivateKeyParameters,
  Ed25519PublicKeyParameters,
  X25519KeyGenerationParameters
}
import org.bouncycastle.crypto.signers.Ed25519Signer
import org.bouncycastle.crypto.util.PublicKeyFactory
import org.bouncycastle.jcajce.interfaces.EdDSAPublicKey
import org.bouncycastle.util.encoders.Hex

import java.io.{ File, FileReader }
import java.nio.file.{ Files, Paths }
import java.security.{ KeyFactory, KeyPairGenerator, Signature }
import javax.crypto.Cipher
import java.security.spec.{ NamedParameterSpec }
import scala.io.Source

class Curve25519Suite extends FunSuite {

  val message = "hello world"
  // https://stackoverflow.com/questions/58583774/how-to-generate-publickey-for-privatekey-in-x25519
  test("generate ed25519 and signature/validate") {

//    val kpg = KeyPairGenerator.getInstance("XDH")
//    kpg.initialize(NamedParameterSpec("X25519"))
//    val pair = kpg.genKeyPair()
//    val privateKey = pair.getPrivate
//    val publicKey = pair.getPublic
//    val cipher = Cipher.getInstance("ECDH")
//    cipher.init(Cipher.ENCRYPT_MODE, privateKey)
//
//    val bytes = cipher.doFinal(message.getBytes)
//
//    cipher.init(Cipher.DECRYPT_MODE, publicKey)
//    val data = cipher.doFinal(bytes)
//    println(new String(data))

//    val kpg = X25519KeyPairGenerator()
//    val param = X25519KeyGenerationParameters(java.security.SecureRandom())
//    kpg.init(param)
//    val pair= kpg.generateKeyPair()
//    val privateKey = pair.getPrivate
//    val publicKey = pair.getPublic

    val kpg = Ed25519KeyPairGenerator()
    kpg.init(Ed25519KeyGenerationParameters(java.security.SecureRandom()))
    val pair = kpg.generateKeyPair()
    val privateKey = pair.getPrivate
    val publicKey = pair.getPublic

    val signer = org.bouncycastle.crypto.signers.Ed25519Signer()
    signer.init(true, privateKey)
    val m = message.getBytes()
    signer.update(m, 0, m.length)
    val signature = signer.generateSignature()
    val verifier = org.bouncycastle.crypto.signers.Ed25519Signer()
    verifier.init(false, publicKey)
    verifier.update(m, 0, m.length)
    val verifyResult = verifier.verifySignature(signature)
    println(s"Result: ${verifyResult}")

//    val kpg: KeyPairGenerator = KeyPairGenerator.getInstance("Ed25519")
//    val kp = kpg.generateKeyPair
//
//    val msg: Array[Byte] = "test_string".getBytes()
//
//    val sig = Signature.getInstance("Ed25519")
//    sig.initSign(kp.getPrivate)
//    sig.update(msg)
//    val signature = sig.sign()
//
//    val verifier = Signature.getInstance("ED25519")
//    verifier.initVerify(kp.getPublic)
//    verifier.update(msg)
//    val result = verifier.verify(signature)
//    println(s"verify result:$result")
  }

  //ed25519 java 原生的，在二进制转译成 PublicKey 时，需要手动写代码转换。且原生导出（DerOutput）为  x509 格式，并不是纯二进制。
  test("ed25519 read from bytes") {

    val kpg = Ed25519KeyPairGenerator()
    kpg.init(Ed25519KeyGenerationParameters(java.security.SecureRandom()))
    val pair = kpg.generateKeyPair()
    val _privateKey = pair.getPrivate.asInstanceOf[Ed25519PrivateKeyParameters]
    val _publicKey = pair.getPublic.asInstanceOf[Ed25519PublicKeyParameters]
    
    val publicKey = Ed25519PublicKeyParameters(_publicKey.getEncoded)
    val privateKey = Ed25519PrivateKeyParameters(_privateKey.getEncoded)

    val msg: Array[Byte] = "test_string".getBytes()

    val sig = Ed25519Signer()

    sig.init(true, privateKey)
    sig.update(msg, 0, msg.length)
    val signature = sig.generateSignature()

    val verifier = Ed25519Signer()
    verifier.init(false, publicKey)
    verifier.update(msg, 0, msg.length)
    val result = verifier.verifySignature(signature)

    println(s"public key size:${_publicKey.getEncoded.length}")
    println(s"private key size:${_privateKey.getEncoded.length}")

    println(s"verify result:$result")

  }

}
