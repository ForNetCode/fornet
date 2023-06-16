package very.util.persistence.quill

import io.getquill.context.jdbc.JdbcContextTypes
import io.getquill.{MappedEncoding, PostgresDialect}
import org.hashids.Hashids
import very.util.security.IntID

trait IDSupport {
  this: JdbcContextTypes[PostgresDialect, _] =>

  given intIDEncode: MappedEncoding[IntID, Int] = MappedEncoding(_.id)
  given intIDDecode(using hashId: Hashids): MappedEncoding[Int, IntID] = MappedEncoding(IntID.apply)
  given intIDListEncoder: MappedEncoding[List[IntID], List[Int]] = MappedEncoding(_.map(_.id))

}
