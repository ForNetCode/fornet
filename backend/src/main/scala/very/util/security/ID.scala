package very.util.security

import org.hashids.Hashids

sealed trait ID[T] {
  def id: T
  def secretId: String
}

case class IntID(id: Int, secretId: String) extends ID[Int]
case class LongID(id: Long, secretId: String) extends ID[Long]

object IntID {
  def apply(id: Int)(using hashId: Hashids): IntID = IntID(id, hashId.encode(id))
  def apply(secretId: String)(using hashId: Hashids): IntID = IntID(hashId.decode(secretId).head.toInt, secretId)

  // given Conversion[IntID, Int] = _.id

  extension (secretId: String)(using hashId: Hashids) {
    def toIntID: IntID = IntID(secretId)
  }
}
