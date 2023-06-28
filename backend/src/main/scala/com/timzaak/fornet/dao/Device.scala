package com.timzaak.fornet.dao

import org.hashids.Hashids
import very.util.security.IntID

import java.time.OffsetDateTime

case class Device(id: IntID, token: String, publicKey: String, createdAt: OffsetDateTime)



class DeviceDao(using quill: DB, hashids: Hashids) {
  import quill.{*, given}
  
  
  def findByIdWithToken(idWithToken:String): Unit = {
    
  }
}
