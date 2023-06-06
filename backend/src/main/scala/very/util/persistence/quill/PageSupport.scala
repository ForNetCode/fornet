package very.util.persistence.quill

import io.getquill.context.jdbc.JdbcContextTypes
import io.getquill.*
import very.util.entity.Pagination

trait PageSupport[+N <: NamingStrategy] {
  this: PostgresJdbcContext[N] =>


  extension[T] (inline q: Query[T]) {
    inline def page(using pagination: Pagination) = {
      q.drop(lift(pagination.offset)).take(lift(pagination.pageSize))
    }

    // warning: sortBy should be split, because PG would report error for count(*)
    inline def pageWithCount(using pagination: Pagination) = {
      (this.run(quote(q.page)), this.run(quote(q.size)))
    }

    inline def pageWithCount(
                              sort: Query[T] => Query[T]
                            )(using pagination: Pagination) = {
      (this.run(quote(sort(q).page)), this.run(quote(q.size)))
    }

    //    inline def pageWithPram(param:T => Boolean)(using pagination:Pagination) = {
    //      q.filter(param).page
    //    }
    inline def single = q.take(1)
  }

}
