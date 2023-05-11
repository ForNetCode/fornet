package com.timzaak.fornet.mqtt.api

import sttp.client3.*
import sttp.client3.logging.slf4j.Slf4jLoggingBackend
import sttp.client3.ziojson.*
import zio.json.*

case class MqttAuthRequest(password: String, publicKey: String)

enum QOS {
  case QOS_0, QOS_1, QOS_2
}
enum PayloadEncode {
  case base64, plain
}
//topic	String	Optional		主题，与 topics 至少指定其中之一
//topics	String	Optional		以 , 分割的多个主题，使用此字段能够同时发布消息到多个主题
//clientid	String	Optional	system	客户端标识符
//payload	String	Required		消息正文
//  encoding	String	Optional	plain	消息正文使用的编码方式，目前仅支持 plain 与 base64 两种
//qos	Integer	Optional	0	QoS 等级
//retain	Boolean	Optional	false	是否为保留消息
case class PublishRequest(
  topic: String,
  topcis: Option[String] = None,
  @jsonField("clientid")
  clientId: Option[String] = None,
  payload: String,
  encoding: Option[String] = None,
  qos: Option[Int] = None,
  retain: Option[Boolean] = None,
)

given JsonEncoder[PublishRequest] = DeriveJsonEncoder.gen

case class SubscribeRequest(
  @jsonField("clientid")
  clientId: String,
  topic: Option[String] = None,
  topics: Option[String] = None,
  qos: Option[Int] = None
)

given JsonEncoder[SubscribeRequest] = DeriveJsonEncoder.gen

case class UnsubscribeRequest(
  @jsonField("clientid")
  clientId: String,
  topic: String,
)

given JsonEncoder[UnsubscribeRequest] = DeriveJsonEncoder.gen

@jsonMemberNames(SnakeCase)
case class SubscribeInfo(
  nodeId: Long,
  @jsonField("clientid")
  clientId: String,
  clientAddr: String,
  topic: String,
  qos: Int,
  share: String,
)
@jsonMemberNames(SnakeCase)
case class ClientInfoResponse(
  nodeId: Long,
  @jsonField("clientid")
  clientId: String,
  username: String,
  protoVer: Int,
  ipAddress: String,
  port: Int,
  connectedAt: String,
  disconnectedAt: Option[String],
  disconnectedReason: Option[String],
  connected: Boolean,
  keepalive: Long,
  cleanStart: Boolean,
  expiryInterval: Long,
  createdAt: String,
  subscriptionsCnt: Int,
  maxSubscriptions: Int,
  inflight: Int,
  maxInflight: Int,
  mqueueLen: Int,
  maxMqueue: Int,
)
given JsonDecoder[ClientInfoResponse] = DeriveJsonDecoder.gen

class RMqttApiClient(host: String) {

  private val backend = Slf4jLoggingBackend(HttpClientSyncBackend())
  private val urlPrefix:String = if(host.endsWith("/")) {host + "api/v1"} else { host + "/api/v1"}

  def isOnline(clientId: String): Boolean = {
    // TODO: handle error
    basicRequest
      .get(uri"${urlPrefix}/clients/$clientId/online")
      .response(asJson[Boolean])
      .send(backend)
      .body
      .getOrElse(false)
  }

  def publish(request: PublishRequest): Boolean = {
    basicRequest
      .post(uri"${urlPrefix}/mqtt/publish")
      .body(request)
      .response(asJson[String])
      .send(backend)
      .is200
  }

  def subscribe(request: SubscribeRequest) = {
    basicRequest
      .post(uri"${urlPrefix}/mqtt/subscribe")
      .body(request)
      .response(asJson[Map[String, Boolean]])
      .send(backend)

  }

  def unsubscribe(request: UnsubscribeRequest): Boolean = {
    basicRequest
      .post(uri"${urlPrefix}/mqtt/unsubscribe")
      .body(request)
      .response(asJson[Boolean])
      .send(backend)
      .is200
  }

  def getClientInfo(
    clientId: String
  ): Either[ResponseException[String, String], ClientInfoResponse] = {
    basicRequest
      .get(uri"${urlPrefix}/clients/$clientId")
      .response(asJson[ClientInfoResponse])
      .send(backend)
      .body
  }

  /*
  def getSubscribeInfo(clientId: String) = {
    basicRequest.get(uri"${url}/subscriptions/$clientId")
      .send(backend)
  }
  */
}
