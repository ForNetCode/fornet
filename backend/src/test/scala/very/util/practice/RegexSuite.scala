package very.util.practice

import munit.FunSuite
import scala.util.matching.Regex

class RegexSuite extends FunSuite {
  private val networkTopicPattern = """^network/(\w+)$""".r
  test("regex") {
    val ID = "ewf014XF"
    val topic = s"network/xxx"
    topic match {
      case s"network/${ID}" => println(s"xx:$ID")
      case networkTopicPattern(secretId) =>
        println(secretId)
      case _ => println("should not come here")
    }


  }
}
