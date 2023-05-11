package very.util.web

import com.typesafe.scalalogging.Logger

import scala.util.{Failure, Try}
trait LogSupport {
  lazy val logger: Logger =
    com.typesafe.scalalogging.Logger(getClass.getName.stripSuffix("$"))

  inline def logTry[T](inline errorMessage: String)(inline func: T): Try[T] = {
    val result = Try(func)
    result match {
      case Failure(exception) =>
        logger.warn(errorMessage, exception)
      case _ =>
    }
    result
  }
}
