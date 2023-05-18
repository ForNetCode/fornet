package com.timzaak.fornet.service

import com.timzaak.fornet.dao.*
import inet.ipaddr.IPAddressString
import org.hashids.Hashids

class NodeService(nodeDao: NodeDao)(using quill: DB, hashId: Hashids) {
  import quill.{*, given}

  // TODO: check if online
  def getAllRelativeNodes(node: Node): List[Node] = {
    val nodeIp = IPAddressString(node.ip)
    val availableRelayNodes =
      nodeDao.getAllAvailableNodes(node.networkId, node.id, NodeType.Relay)
    availableRelayNodes.filter { rNode =>
      IPAddressString(rNode.ip).prefixContains(nodeIp)
    }.toList ++ (node.nodeType match {
      case NodeType.Relay =>
        // all relay/ client ip in relay range
        val clientNodes = nodeDao
          .getAllAvailableNodes(node.networkId, node.id, NodeType.Client)
        clientNodes
          .filter(cNode => nodeIp.prefixContains(IPAddressString(cNode.ip)))
          .toList
      case NodeType.Client =>
        // all relay contains client Ip
        List.empty
    })

  }
  def getAllRelativeNodes(node:Node, nodes:List[Node])= {
    
    //TODO
  }

}
