package com.timzaak.fornet.pubsub

import com.timzaak.fornet.dao.{NetworkDao, *}
import com.timzaak.fornet.grpc.convert.EntityConvert
import com.timzaak.fornet.protobuf.config.{NetworkStatus as PNetworkStatus, NodeStatus as PNodeStatus, NodeType as PNodeType, *}
import com.timzaak.fornet.service.NodeService
import com.typesafe.scalalogging.LazyLogging
import org.hashids.Hashids
import very.util.executor.ScheduledExecutor
import very.util.security.{IntID, TokenID}

class NodeChangeNotifyService(
  nodeDao: NodeDao,
  networkDao: NetworkDao,
  deviceDao: DeviceDao,
  // connectionManager: ConnectionManager,
  connectionManager: MqttConnectionManager,
  nodeService: NodeService,
)(using quill: DB, hashid: Hashids) extends LazyLogging {

  import quill.{*, given}
  private lazy val scheduler = ScheduledExecutor(2)

  def nodeInfoChangeNotify(oldNode: Node, setting: NodeSetting, network: Network) = {
    // TODO: FIXIT
    val networkId = network.id.secretId


    val relativeNodes = nodeService.getAllRelativeNodes(oldNode)
    val fixedNode = oldNode.copy(setting = setting)
    val deviceMap = deviceDao.getAllDevices(oldNode.deviceId::relativeNodes.map(_.deviceId))
    // notify self change
    val wrConfig: WRConfig =
      EntityConvert.nodeToWRConfig(fixedNode, network, relativeNodes, deviceMap)
    val device = deviceMap(fixedNode.deviceId.id)
    connectionManager.sendClientMessage(
      oldNode.networkId,
      device.tokenID,
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
                changePeer = Some(EntityConvert.toPeer(fixedNode, network, device.publicKey))
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
      val allRelativeNodes = nodeService.getNetworkAllRelativeNodes(nodes)
      val deviceIds = allRelativeNodes.map(_::_).flatMap(_.map(_.deviceId)).distinctBy(_.id)
      val deviceMap = deviceDao.getAllDevices(deviceIds)
      for ((node, relativeNodes) <-allRelativeNodes) {
        val wrConfig = EntityConvert.nodeToWRConfig(node, newNetwork, relativeNodes, deviceMap)
        val deviceTokenId = deviceMap(node.id.id).tokenID
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
    device: Device,
    oldStatus: NodeStatus,
    status: NodeStatus,
  ) = {
    import NodeStatus.*
    val networkId = node.networkId.secretId
    // notify self node status change
    connectionManager.sendClientMessage(
      node.networkId,
      device.tokenID,
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
                removePublicKey = Some(device.publicKey)
              )
            )
          )
        )
        if(status == NodeStatus.Delete|| status == NodeStatus.Forbid) {
          import concurrent.duration.DurationInt
          scheduler.delayExecution({
            connectionManager.kickOffNetwork(device.tokenID, node.networkId)
          })(5.seconds)
        }

      case (_, Normal) =>
        val network = networkDao.findById(node.networkId).get
        val peer = EntityConvert.toPeer(node, network, device.publicKey)

        val notifyNodes = nodeService.getAllRelativeNodes(node)
        val deviceMap = deviceDao.getAllDevices(node.deviceId::notifyNodes.map(_.deviceId))
        connectionManager.sendClientMessage(
          node.networkId,
          device.tokenID,
          ClientMessage(
            networkId = networkId,
            ClientMessage.Info.Config(
              EntityConvert.nodeToWRConfig(node, network, notifyNodes, deviceMap)
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
