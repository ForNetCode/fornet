package very.util.persistence.quill

import io.getquill.context.jdbc.JdbcContextTypes
import io.getquill.PostgresDialect
import org.postgresql.util.PGobject
import zio.json.*

trait DBSerializer
trait ZIOJsonSupport {
  this: JdbcContextTypes[PostgresDialect, _] =>

  given encodeJsonb[T<:DBSerializer](using JsonEncoder[T]): Encoder[T] = {
    encoder(
      java.sql.Types.OTHER,
      (index, value, row) => {
        val jsonObject = new PGobject()
        jsonObject.setType("jsonb")
        jsonObject.setValue(value.toJson)
        row.setObject(index, jsonObject)
      }
    )
  }

  given decodeJsonb[T<:DBSerializer](using JsonDecoder[T]):Decoder[T] =
    decoder{(index,row, session) =>
      val data = row.getString(index)
      data.fromJson[T].toOption.get
    }
}
