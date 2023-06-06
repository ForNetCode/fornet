package com.timzaak.fornet.dao

import com.timzaak.fornet.dao.Network
import io.getquill.*
import io.getquill.context.jdbc.{ Decoders, Encoders }
import very.util.persistence.quill.{ IDSupport, PageSupport, ZIOJsonSupport }
//import org.json4s.Extraction
//import org.json4s.JsonAST.JValue
import very.util.entity.Pagination

import java.time.{ LocalDateTime, OffsetDateTime }
import java.util.Calendar

class DB extends PostgresJdbcContext(SnakeCase, "database") with ZIOJsonSupport with IDSupport with PageSupport[SnakeCase] {

  given encodeOffsetDateTime: Encoder[OffsetDateTime] =
    encoder(
      java.sql.Types.TIMESTAMP_WITH_TIMEZONE,
      (index, value, row) => row.setObject(index, value)
    )

  given decodeOffsetDateTime: Decoder[OffsetDateTime] =
    decoder((index, row, _) => row.getObject(index, classOf[OffsetDateTime]))

  // import org.json4s.jvalue2extractable

//  private inline def encodeJValueEntity[T]:MappedEncoding[T,JValue] = MappedEncoding[T,JValue](v => Extraction.decompose(v)(formats))
//  private inline def decodeJValueEntity[T](implicit mf:scala.reflect.Manifest[T]):MappedEncoding[JValue, T] = MappedEncoding[JValue,T](_.extract[T])
//
//
//  given encodeNetworkSetting:MappedEncoding[NetworkSetting, JValue] = MappedEncoding[NetworkSetting, JValue](v => Extraction.decompose(v)(formats))
//
//  given decodeNetworkSetting:MappedEncoding[JValue, NetworkSetting] = MappedEncoding[JValue, NetworkSetting](_.extract[NetworkSetting])
//
//  given encodeNodeSetting:MappedEncoding[NodeSetting, JValue] = MappedEncoding(v => Extraction.decompose(v)(formats)) // encodeJValueEntity[NodeSetting]
//  given decodeNodeSetting:MappedEncoding[JValue,NodeSetting] = decodeJValueEntity[NodeSetting]

  given encodeNodeType: MappedEncoding[NodeType, Int] = MappedEncoding(
    _.ordinal
  )

  given decodeNodeType: MappedEncoding[Int, NodeType] = MappedEncoding(
    NodeType.fromOrdinal
  )

  given encodeNodeStatus: MappedEncoding[NodeStatus, Int] = MappedEncoding(
    _.ordinal
  )

  given decodeNodeStatus: MappedEncoding[Int, NodeStatus] = MappedEncoding(
    NodeStatus.fromOrdinal
  )

  given encodeLinkType: MappedEncoding[LinkType, Int] = MappedEncoding(
    _.ordinal
  )

  given decodeLinkType: MappedEncoding[Int, LinkType] = MappedEncoding(
    LinkType.fromOrdinal
  )

  given encodeNetworkStatus: MappedEncoding[NetworkStatus, Int] =
    MappedEncoding(_.ordinal)
  given decodeNetworkStatus: MappedEncoding[Int, NetworkStatus] =
    MappedEncoding(NetworkStatus.fromOrdinal)
  
}

//@main def testQuill = {
//
//  import QuillContext.{given, *}
//  val r = QuillContext.run {
//    quote {
//      query[Network]
//      //query[Network].insert(_.name -> "test1", _.addressRange -> "10.0.0.0/16", _.setting -> lift(NetworkSetting("test", "test2")))
//    }
//  }
//  println(r)
//}
