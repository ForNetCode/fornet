package com.timzaak.fornet.pubsub

import com.timzaak.fornet.mqtt.api.{ PublishRequest, RMqttApiClient }
import com.timzaak.fornet.protobuf.config.{ ClientMessage, NetworkMessage }
import org.hashids.Hashids
import scalapb.GeneratedMessage
import very.util.security.IntID
import very.util.web.LogSupport

import java.util.Base64
import scala.util.Try

class MqttConnectionManager(
  mqttApiClient: RMqttApiClient
)(using hashid: Hashids)
  extends LogSupport {

  private def encodeMessage(message: GeneratedMessage) =
    Base64.getEncoder.encodeToString(message.toByteArray)

  def sendMessage(networkId: IntID, message: NetworkMessage, retain: Option[Boolean] = Some(false)): Try[Boolean] = {
    logTry(s"send message[Network:${networkId.id}] failure")(
      mqttApiClient.publish(
        PublishRequest(
          payload = encodeMessage(message),
          qos = Some(1),
          encoding = Some("base64"),
          topic = s"network/${networkId.secretId}",
          retain = retain,
        )
      )
    )
  }
  def sendMessage(
    networkId: IntID,
    nodeId: IntID,
    publicKey: String,
    message: ClientMessage,
    retain: Option[Boolean] = Some(false),
  ): Try[Boolean] = {
    logTry(s"send message[Client:${networkId.id}-${nodeId.id}] failure")(
      mqttApiClient.publish(
        PublishRequest(
          payload = encodeMessage(message),
          clientId = Some(publicKey),
          qos = Some(1),
          encoding = Some("base64"),
          topic = s"client/${nodeId.secretId}",
          retain,
        )
      )
    )
  }

  def isOnline(publicKey: String): Boolean = mqttApiClient.isOnline(publicKey)
}
