package com.timzaak.fornet.dao

import org.hashids.Hashids
import very.util.security.{IntID, TokenID}

import java.time.OffsetDateTime

case class Device(id: IntID, token: String, publicKey: String, createdAt: OffsetDateTime)

import io.getquill.*
class DeviceDao(using quill: DB, hashids: Hashids) {
  import quill.{*, given}

  def findByIdWithToken(tokenIdStr: String): Unit = {
    val tokenId = TokenID(tokenIdStr)
    quill
      .run(quote(query[Device]).filter(v => v.id == lift(tokenId.intId) && v.token == lift(tokenId.token)).single)
      .head
  }
}
