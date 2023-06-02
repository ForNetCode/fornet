package very.util.web.json

import jakarta.servlet.http.HttpServletRequest
import org.hashids.Hashids
import org.scalatra.*
import very.util.security.IntID
import zio.json.*
import zio.json.ast.Json

import scala.util.{ Failure, Success, Try }

trait ZIOJsonSupport extends ApiFormats {

  private def parsedBody[T](action: T => Any)(implicit request: HttpServletRequest, decoder: JsonDecoder[T]): Any = {
    request.body.fromJson[T] match {
      case Right(v) => action(v)
      case Left(msg) =>
        contentType = formats("txt")
        BadRequest(msg)
    }
  }

  override protected def renderPipeline: RenderPipeline = ({
    case DefaultJResponse(None) | None =>
      contentType = formats("txt")
      response.status = 404
      response.writer.write("Not Found")
    case Some(v: String) =>
      contentType = formats("txt")
      response.writer.write(v)
    case v: JResponse[?] =>
      contentType = formats("json")
      response.writer.write(v.toJson)
  }: RenderPipeline) orElse super.renderPipeline

  inline def jGet[T](transformers: RouteTransformer*)(
    action: => T | (List[T], Long)
  )(using JsonEncoder[T]): Route = get(transformers: _*)(action.r)

  inline def jPost[T](transformers: RouteTransformer*)(
    action: => T
  )(using JsonEncoder[T]): Route = post(transformers: _*)(action.r)

  inline def jPost[R](transformers: RouteTransformer*)(
    action: R => Any
  )(using JsonDecoder[R]): Route = post(transformers: _*) {
    parsedBody[R](action)
  }

  inline def jPost2[R, T](transformers: RouteTransformer*)(
    action: R => T
  )(using JsonDecoder[R], JsonEncoder[T]): Route = post(transformers: _*) {

    parsedBody[R](p => action(p).r)
  }
  inline def jPut[T](transformers: RouteTransformer*)(
    action: => T
  )(using JsonEncoder[T]): Route = put(transformers: _*)(action.r)

  inline def jPut[R](transformers: RouteTransformer*)(
    action: R => Any
  )(using JsonDecoder[R]): Route = put(transformers: _*) {
    parsedBody[R](action)
  }

  inline def jPut2[R, T](transformers: RouteTransformer*)(
    action: R => T
  )(using JsonDecoder[R], JsonEncoder[T]): Route = put(transformers: _*) {
    parsedBody[R](p => action(p).r)
  }

  extension [T](result: T | (List[T], Long))(using JsonEncoder[T]) {
    inline def r: JResponse[T] = result match {
      case (body: List[T], count: Long) => PageJResponse(count, body)
      case body: T                      => DefaultJResponse(body)
    }
  }
}

trait JResponse[T] {
  def toJson: String
}
case class DefaultJResponse[T](body: T)(using JsonEncoder[T]) extends JResponse[T] {
  def toJson: String = body.toJson
}
case class PageJResponse[T](total: Long, body: List[T])(using JsonEncoder[T]) extends JResponse[T] {
  def toJson: String =
    Json.Obj("total" -> Json.Num(total), "list" -> body.toJsonAST.getOrElse(Json.Arr())).toJson

}
case class JsonResponse[T](body: T)(using JsonEncoder[T]) {
  def toJson: String = body.toJson
}

given intIDDecoder(using hashId: Hashids): JsonDecoder[IntID] = JsonDecoder[String].mapOrFail { v =>
  Try(IntID.apply(v)) match {
    case Success(v) => Right(v)
    case Failure(_) => Left("Invalid ID")
  }
}
given intIDEncoder(using hashId: Hashids): JsonEncoder[IntID] = JsonEncoder.int.contramap(_.id)
