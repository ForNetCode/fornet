package com.timzaak.fornet.grpc.convert

import com.timzaak.fornet.dao.{Device, Network, Node}
import com.timzaak.fornet.protobuf.config.{Interface, Peer, WRConfig}

object EntityConvert {

  def toPeers(nodes: List[Node], network: Network, deviceMap: Map[Int, Device]) = {
    nodes.map(node => toPeer(node, network, deviceMap(node.deviceId.id).publicKey))
  }

  def toPeer(node: Node, network: Network, publicKey:String) = {
    val nodeSetting = node.setting
    val defaultPort = network.setting.port
    val defaultKeepAlive = network.setting.keepAlive

    Peer(
      endpoint = nodeSetting.endpoint.map(v => s"$v:${nodeSetting.port.getOrElse(defaultPort)}"),
      allowedIp = Seq(node.peerAllowedIp),
      publicKey = publicKey,
      address = Seq(node.peerAddress),
      persistenceKeepAlive = nodeSetting.keepAlive.getOrElse(defaultKeepAlive),
    )
  }

  def nodeToWRConfig(
    node: Node,
    network: Network,
    relativeNodes: List[Node],
    deviceMap:Map[Int, Device],
  ): WRConfig = {
    val setting = node.setting
    val nSetting = network.setting

    WRConfig(
      interface = Some(
        Interface(
          name = Some(node.name),
          address = Seq(node.address(network)),
          listenPort = setting.port.getOrElse(nSetting.port),
          dns = setting.dns.orElse(nSetting.dns).getOrElse(Seq.empty),
          mtu = Some(setting.mtu.getOrElse(nSetting.mtu)),
          postUp = setting.postUp,
          postDown = setting.postDown,
          protocol = nSetting.protocol.gRPCProtocol,
        ),
      ),
      peers = toPeers(relativeNodes.filter(_.id != node.id), network, deviceMap),
      `type` = node.nodeType.gRPCNodeType,
    )
  }
}
