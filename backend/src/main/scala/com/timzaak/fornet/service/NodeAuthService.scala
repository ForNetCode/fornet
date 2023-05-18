package com.timzaak.fornet.service

import com.timzaak.fornet.dao.{DB, NodeDao}
import com.timzaak.fornet.entity.{NodeIdentity, PublicKey}
import com.timzaak.fornet.protobuf.auth.{EncryptRequest, InviteConfirmRequest}
import io.grpc.Status
import org.hashids.Hashids
import zio.json.{DeriveJsonCodec, JsonCodec}

import scala.util.Try

case class GRPCAuthRequest(
  publicKey: PublicKey,
  networkId: String,
  nonce: String,
  timestamp: Long,
  sign: String
)
object GRPCAuthRequest {
  given JsonCodec[GRPCAuthRequest] = DeriveJsonCodec.gen
}

case class GRPCAuth(publicKey: PublicKey, networkId: Int)

//TODO: This should not be service, change it to object
class NodeAuthService(using hashId: Hashids) {
  def validate(
    encrypt: EncryptRequest,
    params: Seq[String],
  ): Boolean = {
    import encrypt.*
    val plainText =
      params.appendedAll(Seq(nonce, timestamp.toString)).mkString("|")
    Try {
      PublicKey(publicKey).validate(plainText, signature)
    }.getOrElse(false)
  }
}

