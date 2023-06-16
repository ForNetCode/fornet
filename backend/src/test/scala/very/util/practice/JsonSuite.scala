package very.util.practice

//import com.fasterxml.jackson.annotation.JsonFormat
import com.timzaak.fornet.dao.{NetworkSetting, NodeSetting}
import munit.FunSuite

import zio.json.*
import zio.json.ast.Json

enum TestEnum {
  case A,B
}
object TestEnum {
  given JsonCodec[TestEnum] = DeriveJsonCodec.gen
}




class JsonSuite extends FunSuite {
  //given encodeNetworkSetting:MappedEncoding[NetworkSetting, JValue] = MappedEncoding[NetworkSetting, JValue](v => Extraction.decompose(v)(formats))
  given JsonDecoder[NodeSetting] =
    DeriveJsonDecoder.gen[NodeSetting]
  given JsonEncoder[NodeSetting] = DeriveJsonEncoder.gen[NodeSetting]

//  test("jsonFormat, encode") {
//    val a = Extraction.decompose(NodeSetting())(org.json4s.DefaultFormats)
//    println(a)
//  }
  test("zio-json") {

    val a = NodeSetting(port = Some(1)).toJson

    println(a)

    val b = a.fromJson[NodeSetting]
    println(b)
  }

  test("enum parse") {
    val a = TestEnum.B.toJson
    println(a)
    println(a.fromJson[TestEnum])
  }

  test("json default value") {
    val str = """{"mtu": 1420, "port": 51820, "keepAlive": 30}"""
    println(s"result: ${str.fromJson[NetworkSetting]}")
  }

  /* test("jresponse") {
    val data = PageJResponse(10,List("1","2"))
    println(data.toJson)
  } */
}
