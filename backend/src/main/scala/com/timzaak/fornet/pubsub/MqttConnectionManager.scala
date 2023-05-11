package com.timzaak.fornet.pubsub

import com.timzaak.fornet.mqtt.api.{PublishRequest, RMqttApiClient}
import com.timzaak.fornet.protobuf.config.{ClientMessage, NetworkMessage}
import org.hashids.Hashids
import scalapb.GeneratedMessage
import very.util.web.LogSupport

import java.util.Base64
import scala.util.Try

class MqttConnectionManager(
  mqttApiClient: RMqttApiClient
)(using hashid: Hashids)
  extends LogSupport {

  private def encodeMessage(message: GeneratedMessage) =
    Base64.getEncoder.encodeToString(message.toByteArray)

  def sendMessage(networkId: Int, message: NetworkMessage): Try[Boolean] = {
    logTry(s"send message[Network:$networkId] failure")(
      mqttApiClient.publish(
        PublishRequest(
          payload = encodeMessage(message),
          qos = Some(1),
          encoding = Some("base64"),
          topic = s"network/${hashid.encode(networkId)}"
        )
      )
    )
  }
  def sendMessage(
    networkId: Int,
    nodeId: Int,
    publicKey: String,
    message: ClientMessage,
    retain: Option[Boolean] = Some(false),
  ): Try[Boolean] = {
    logTry(s"send message[Client:${networkId}-${nodeId}] failure")(
      mqttApiClient.publish(
        PublishRequest(
          payload = encodeMessage(message),
          clientId = Some(publicKey),
          qos = Some(1),
          encoding = Some("base64"),
          topic = "client",
          retain,
        )
      )
    )
  }

  def isOnline(publicKey: String): Boolean = mqttApiClient.isOnline(publicKey)
}
