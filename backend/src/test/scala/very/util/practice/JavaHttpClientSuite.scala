package very.util.practice

import munit.FunSuite

import java.net.{URI, URL}
import java.net.http.{HttpClient, HttpRequest, HttpResponse}
import java.net.http.HttpRequest.BodyPublishers

class JavaHttpClientSuite extends FunSuite {
  test("uri") {
    val uri = URI("http://keycloak-dev.fornet.com/realms/fornet/protocol/openid-connect/auth/device")
    println("ok")
    val request = HttpRequest
      .newBuilder()
      .uri(uri)
      .POST(
        BodyPublishers.ofString(
          s"client_id=fornet"
        )
      )
      .build()
    val client = HttpClient.newHttpClient()
    val response = client.send(request, HttpResponse.BodyHandlers.ofString())
    println(response.uri())
  }
}
