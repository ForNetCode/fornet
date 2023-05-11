package com.timzaak.fornet.dao

import com.timzaak.fornet.entity.NodeIdentity

import very.util.grpc.auth.SessionProvider

import java.time.OffsetDateTime
import java.util.UUID

case class Session(
  sessionId: String,
  networkId: Int,
  nodeId: Int,
  createdAt: OffsetDateTime
)

import io.getquill.*

class SessionDao(using quill: DB) extends SessionProvider[NodeIdentity] {
  import quill.{ given, * }

  override def createSession(data: NodeIdentity): String = {
    val sessionId = UUID.randomUUID().toString
    quill.run {
      quote {
        query[com.timzaak.fornet.dao.Session].insert(
          _.sessionId -> lift(sessionId),
          _.nodeId -> lift(data.nodeId),
          _.networkId -> lift(data.networkId),
        )
      }
    }
    sessionId
  }

  override def getData(sessionId: String): Option[NodeIdentity] = {
    quill
      .run {
        quote {
          query[com.timzaak.fornet.dao.Session]
            .filter(_.sessionId == lift(sessionId))
            .single
        }
      }
      .headOption
      .map(v => NodeIdentity(networkId = v.networkId, nodeId = v.nodeId))
  }
}
