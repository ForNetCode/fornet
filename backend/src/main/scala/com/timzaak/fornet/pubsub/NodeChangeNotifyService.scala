package com.timzaak.fornet.pubsub

import com.timzaak.fornet.dao.*
import com.timzaak.fornet.grpc.convert.EntityConvert
import com.timzaak.fornet.protobuf.config.{ NodeStatus as PNodeStatus, * }
import com.timzaak.fornet.service.NodeService
import org.hashids.Hashids

class NodeChangeNotifyService(
  nodeDao: NodeDao,
  networkDao: NetworkDao,
  // connectionManager: ConnectionManager,
  connectionManager: MqttConnectionManager,
  nodeService: NodeService,
)(using quill: DB, hashid: Hashids) {

  import quill.{ *, given }

  def nodeInfoChangeNotify(oldNode: Node, setting: NodeSetting) = {
    // TODO: FIXIT

    val network = networkDao.findById(oldNode.networkId).get
    val networkId = network.id.secretId

    val relativeNodes = nodeService.getAllRelativeNodes(oldNode)
    val fixedNode = oldNode.copy(setting = setting)
    // notify self change
    val wrConfig: WRConfig =
      EntityConvert.nodeToWRConfig(fixedNode, network, relativeNodes)

    connectionManager.sendMessage(
      oldNode.networkId,
      oldNode.id,
      oldNode.publicKey,
      ClientMessage(networkId = networkId, ClientMessage.Info.Config(wrConfig))
    )
    fixedNode.nodeType match {
      case NodeType.Client =>
      // notify relay nodes in network that client change.
      // only keep alive matter
      case NodeType.Relay =>
        // notify other nodes in network that relay change.
        connectionManager.sendMessage(
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

  def nodeStatusChangeNotify(
    node: Node,
    oldStatus: NodeStatus,
    status: NodeStatus,
  ) = {
    import NodeStatus.*
    val networkId = node.networkId.secretId
    // notify self node status change
    connectionManager.sendMessage(
      node.networkId,
      node.id,
      node.publicKey,
      ClientMessage(
        networkId = networkId,
        ClientMessage.Info.Status(status.gRPCNodeStatus)
      )
    )

    (oldStatus, status) match {
      case (Normal, _) =>
        connectionManager.sendMessage(
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

        connectionManager.sendMessage(
          node.networkId,
          NetworkMessage(
            networkId = networkId,
            NetworkMessage.Info.Peer(
              PeerChange(addPeer = Some(peer))
            )
          )
        )

        val notifyNodes = nodeService.getAllRelativeNodes(node)

        connectionManager.sendMessage(
          node.networkId,
          node.id,
          node.publicKey,
          ClientMessage(
            networkId = networkId,
            ClientMessage.Info.Config(
              EntityConvert.nodeToWRConfig(node, network, notifyNodes)
            )
          )
        )
      case _ =>
      // do nothing.
    }

  }
}
