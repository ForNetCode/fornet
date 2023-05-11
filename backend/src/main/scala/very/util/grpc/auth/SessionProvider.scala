package very.util.grpc.auth

import io.grpc.Context

trait SessionProvider[V] {
  def createSession(data:V):String
  def getData(sessionId:String):Option[V]
  val SESSION_KEY:Context.Key[V] = Context.key("session")
}

