package very.util.web

import com.timzaak.fornet.dao.DB

case class Pagination(page: Int, pageSize: Int) {
  assert(pageSize <= 50)
  assert(page > 0)
  def offset: Int = (page - 1) * pageSize
  def limit: Int = pageSize

}

trait PaginationSupport { this: org.scalatra.ScalatraBase =>

  private def page = params.get("page").fold(1)(_.toInt)

  private def pageSize = params.get("pageSize").fold(10)(_.toInt)

  given pagination: Pagination = Pagination(page, pageSize)

  inline def search[T](
                        arguments: Map[String, String => T => Boolean]
                      ): Iterable[T => Boolean] = {
    for {
      (k, func) <- arguments
      value <- params.get(k) if value.nonEmpty
    } yield func(value)
    
  }
}
