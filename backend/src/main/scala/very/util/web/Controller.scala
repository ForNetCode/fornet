package very.util.web

import com.typesafe.scalalogging.LazyLogging
import org.scalatra.json.JacksonJsonSupport
import org.scalatra.*
//import org.json4s.Formats
import org.scalatra.i18n.I18nSupport
import very.util.web.json.ZIOJsonSupport
import very.util.web.validate.ValidationExtra
import zio.NonEmptyChunk
import zio.prelude.Validation
class Controller //(using val jsonFormats: Formats)
  extends ScalatraServlet
  with ZIOJsonSupport
  with I18nSupport
  with ValidationExtra
  with PaginationSupport
  with LazyLogging {
  override def defaultFormat: Symbol = Symbol("txt")
  def badResponse(msg: String): ActionResult = {
    contentType = formats("txt")
    BadRequest(msg)
  }
  def serverError(msg: String): ActionResult = {
    contentType = formats("txt")
    InternalServerError(msg)
  }

  errorHandler = {
    case _: org.json4s.MappingException | _: java.lang.NumberFormatException | _: java.lang.AssertionError =>
      badResponse(messages("error.parameter_error"))
    case t =>
      logger.error("errorHandler", t)
      serverError(messages("error.internal_server_error"))
  }
  protected override def renderPipeline: RenderPipeline = ({
    case Validation.Success(_, value) => super.renderPipeline(value)
    case Validation.Failure(_, data) =>
      val info = badResponse(i18n("error.request_error")(data.mkString("\n")))
      super.renderPipeline(info)
  }: RenderPipeline) orElse super.renderPipeline

  /*
  def created(id: Long): ActionResult = {
    contentType = formats("json")
    Created(s"""{"id":$id}""")
  }
  */
  def created(id: very.util.security.ID[_]): ActionResult = {
    contentType = formats("json")
    Created(s"""{"id":"${id.secretId}"}""")
  }
}
