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

class NodeAuthService(using hashId: Hashids) {

  // import quill.{ given, * }

  /* def validate(grpcAuth: GRPCAuth): Either[Status, NodeIdentity] = {
    if (NodeAuthService.validate(grpcAuth)) {
      nodeDao
        .findIdByPublicKey(grpcAuth.publicKey.key, grpcAuth.networkId)
        .map(NodeIdentity(grpcAuth.networkId, _))
        .toRight(
          Status.NOT_FOUND.withDescription("Could not find Node")
        )
    } else {
      Left(Status.INVALID_ARGUMENT.withDescription("Invalid auth"))
    }
  } */

  @deprecated
  def validate(grpcAuth: GRPCAuthRequest): Option[Int] = {
    import grpcAuth.*
    val plainText = s"$timestamp-$networkId-$nonce"
    if (publicKey.validate(plainText, sign)) {
      val hashIds = hashId.decode(networkId)
      if (hashIds.size == 1) {
        Some(hashIds.head.toInt)
      } else {
        None
      }
    } else {
      None
    }
  }

  def validate2(
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
