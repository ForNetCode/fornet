package com.timzaak.fornet.pubsub

import com.timzaak.fornet.dao.{NetworkDao, *}
import com.timzaak.fornet.grpc.convert.EntityConvert
import com.timzaak.fornet.protobuf.config.{NetworkStatus as PNetworkStatus, NodeStatus as PNodeStatus, NodeType as PNodeType, *}
import com.timzaak.fornet.service.NodeService
import org.hashids.Hashids
import very.util.security.{IntID, TokenID}

class NodeChangeNotifyService(
  nodeDao: NodeDao,
  networkDao: NetworkDao,
  deviceDao: DeviceDao,
  // connectionManager: ConnectionManager,
  connectionManager: MqttConnectionManager,
  nodeService: NodeService,
)(using quill: DB, hashid: Hashids) {

  import quill.{ *, given }

  def nodeInfoChangeNotify(oldNode: Node, setting: NodeSetting, network: Network) = {
    // TODO: FIXIT
    val networkId = network.id.secretId


    val relativeNodes = nodeService.getAllRelativeNodes(oldNode)
    val fixedNode = oldNode.copy(setting = setting)
    // notify self change
    val wrConfig: WRConfig =
      EntityConvert.nodeToWRConfig(fixedNode, network, relativeNodes)
    val deviceTokenId = deviceDao.getTokenId(oldNode.deviceId).get
    connectionManager.sendClientMessage(
      oldNode.networkId,
      deviceTokenId,
      ClientMessage(networkId = networkId, ClientMessage.Info.Config(wrConfig))
    )
    fixedNode.nodeType match {
      case NodeType.Client =>
      // notify relay nodes in network that client change.
      // only keep alive matter
      case NodeType.Relay =>
        // notify other nodes in network that relay change.
        connectionManager.sendNetworkMessage(
          fixedNode.networkId,
          NetworkMessage(
            networkId = networkId,
            NetworkMessage.Info.Peer(
              PeerChange(
                changePeer = Some(EntityConvert.toPeer(fixedNode, network))
              )
            )
          )
        )
    }
  }

  // network must be in normal status
  def networkSettingChange(oldNetwork: Network, newNetwork: Network): Unit = {
    // only care about protocol, others will trigger push in future version.(after solved async push)
    if (oldNetwork.setting.protocol != newNetwork.setting.protocol && newNetwork.status == NetworkStatus.Normal) {
      val nodes = nodeDao.getAllAvailableNodes(oldNetwork.id).toList
      val deviceMap = deviceDao.getTokenIds(nodes.map(_.id))
      for ((node, relativeNodes) <- nodeService.getNetworkAllRelativeNodes(nodes)) {
        val wrConfig = EntityConvert.nodeToWRConfig(node, newNetwork, relativeNodes)
        val deviceTokenId = deviceMap(node.id.id)
        // this would trigger all nodes restart.
        connectionManager.sendClientMessage(
          node.networkId,
          deviceTokenId,
          ClientMessage(networkId = newNetwork.id.secretId, ClientMessage.Info.Config(wrConfig))
        )
      }

    }
  }

  // PS: Network would never recover from delete status
  def networkDeleteNotify(networkId: IntID): Unit = {
    connectionManager.sendNetworkMessage(
      networkId,
      NetworkMessage(
        networkId = networkId.secretId,
        NetworkMessage.Info.Status(PNetworkStatus.NETWORK_DELETE)
      )
    )
  }

  def nodeStatusChangeNotify(
    node: Node,
    deviceTokenId: TokenID,
    oldStatus: NodeStatus,
    status: NodeStatus,
  ) = {
    import NodeStatus.*
    val networkId = node.networkId.secretId
    // notify self node status change
    connectionManager.sendClientMessage(
      node.networkId,
      deviceTokenId,
      ClientMessage(
        networkId = networkId,
        ClientMessage.Info.Status(status.gRPCNodeStatus),
      )
    )

    (oldStatus, status) match {
      case (Normal, _) =>
        connectionManager.sendNetworkMessage(
          node.networkId,
          NetworkMessage(
            networkId = networkId,
            NetworkMessage.Info.Peer(
              PeerChange(
                removePublicKey = Some(node.publicKey)
              )
            )
          )
        )

      case (_, Normal) =>
        val network = networkDao.findById(node.networkId).get
        val peer = EntityConvert.toPeer(node, network)

        val notifyNodes = nodeService.getAllRelativeNodes(node)

        connectionManager.sendClientMessage(
          node.networkId,
          deviceTokenId,
          ClientMessage(
            networkId = networkId,
            ClientMessage.Info.Config(
              EntityConvert.nodeToWRConfig(node, network, notifyNodes)
            ),
          )
        )

        connectionManager.sendNetworkMessage(
          node.networkId,
          NetworkMessage(
            networkId = networkId,
            NetworkMessage.Info.Peer(
              PeerChange(addPeer = Some(peer))
            )
          )
        )
      case _ =>
      // do nothing.
    }
  }
}
