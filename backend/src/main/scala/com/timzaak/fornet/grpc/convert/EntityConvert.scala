package com.timzaak.fornet.grpc.convert

import com.timzaak.fornet.dao.{Network, Node}
import com.timzaak.fornet.protobuf.config.{Interface, Peer, WRConfig}

object EntityConvert {

  def toPeers(nodes: List[Node], network: Network) = {
    import com.timzaak.fornet.protobuf.config.Peer
    nodes.map(toPeer(_, network))
  }

  def toPeer(node: Node, network: Network) = {
    val nodeSetting = node.setting
    val defaultPort = network.setting.port
    val defaultKeepAlive = network.setting.keepAlive

    Peer(
      endpoint = nodeSetting.endpoint.map(v =>
        s"$v:${nodeSetting.port.getOrElse(defaultPort)}"
      ),
      allowedIp = Seq(node.peerAddress),
      publicKey = node.publicKey,
      persistenceKeepAlive = nodeSetting.keepAlive.getOrElse(defaultKeepAlive),
    )
  }

  def nodeToWRConfig(
    node: Node,
    network: Network,
    relativeNodes: List[Node]
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
        ),
      ),
      peers = toPeers(relativeNodes.filter(_.id != node.id), network)
    )
  }
}
