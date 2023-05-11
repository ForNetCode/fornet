package very.util.practice

import munit.FunSuite
import org.hashids.Hashids

class HashIdsSuite extends FunSuite {

  test("hashid convert") {
    val hashId1 = new Hashids("dotboring dev salt", 5)
    val hashId2 = new Hashids("fornet dev salt", 5)
    val id = hashId1.decode("beP3W")
    println(s"hashId:$id")
    println(s"encode: ${hashId2.encode(id: _*)}")

  }
}
