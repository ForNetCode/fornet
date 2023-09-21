package com.timzaak.fornet.dao

import org.hashids.Hashids
import very.util.security.{IntID, TokenID}

import java.time.OffsetDateTime

case class Device(id: IntID, token: String, publicKey: String, createdAt: OffsetDateTime) {
  def tokenID: TokenID = TokenID(id, token)
}

import io.getquill.*
class DeviceDao(using quill: DB, hashids: Hashids) {
  import quill.{*, given}

  def findByTokenID(tokenId: TokenID): Option[Device] = {
    quill
      .run(quote(query[Device]).filter(v => v.id == lift(tokenId.intId) && v.token == lift(tokenId.token)).single)
      .headOption
  }
  def findById(id:IntID): Option[Device] = {
    quill
      .run(quote(query[Device]).filter(v => v.id == lift(id)).single)
      .headOption
  }
  
  def getAllDevices(ids:List[IntID]):Map[Int, Device]= {
    if(ids.isEmpty) {
      Map.empty
    } else {
      quill.run(quote(query[Device]).filter(v => liftQuery(ids).contains(v.id))).map(v => v.id.id -> v).toMap
    }
  }
}
