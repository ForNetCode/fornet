package very.util.grpc.auth

import io.grpc.{Context, Contexts, Metadata, ServerCall, ServerCallHandler, ServerInterceptor, Status}
import io.grpc.Metadata.ASCII_STRING_MARSHALLER

//TODO: limit try time..

class SessionServerInterceptor[T](sessionProvider: SessionProvider[T]) extends ServerInterceptor {

  override def interceptCall[ReqT, RespT](call: ServerCall[ReqT, RespT], headers: Metadata, next: ServerCallHandler[ReqT, RespT]): ServerCall.Listener[ReqT] = {
    def error(status:Status) = {
      call.close(status, Metadata())
      new ServerCall.Listener[ReqT]{}
    }
    Option(headers.get[String](SessionServerInterceptor.AUTHORIZATION_METADATA_KEY)) match {
      case None =>
        error(Status.UNAUTHENTICATED.withDescription("Authorization token is missing"))
      case Some(value) if !value.startsWith(SessionServerInterceptor.BEARER_TYPE) =>
        error(Status.UNAUTHENTICATED.withDescription("Unknown authorization type"))
      case Some(value) =>
        val token = value.substring(SessionServerInterceptor.BEARER_TYPE.length).trim()
        sessionProvider.getData(token) match {
          case None => error(Status.UNAUTHENTICATED.withDescription("Authorization could not find token"))
          case Some(data) =>
            val ctx = Context.current().withValue(sessionProvider.SESSION_KEY, data);
            Contexts.interceptCall(ctx, call, headers, next)
        }
    }
  }
}

object SessionServerInterceptor {
  val  AUTHORIZATION_METADATA_KEY:Metadata.Key[String] = Metadata.Key.of("Authorization", ASCII_STRING_MARSHALLER)
  val BEARER_TYPE = "Bearer"

}