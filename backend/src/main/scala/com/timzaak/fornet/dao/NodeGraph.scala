package com.timzaak.fornet.dao


import io.getquill.MappedEncoding
import zio.json.{DeriveJsonCodec, JsonCodec}

import java.time.OffsetDateTime

case class NodeGraph(
  networkId: Int,
  nodeId: Int,
  refId: Int,
  linkType: LinkType,
  createdAt: OffsetDateTime,
)
enum LinkType {
  case ParentToChild, Peer
}
object LinkType {
  given JsonCodec[LinkType] = DeriveJsonCodec.gen

}

object NodeGraph {
  given JsonCodec[NodeGraph] = DeriveJsonCodec.gen
}



import io.getquill.*
class NodeGraphDao(using quill:DB) {
  import quill.{given, *}

  // relativeNodes length may be short than nodeGraphMap  
  def getRelativeNodeIds(networkId:Int, nodeId:Int) = {
    val nodeGraph = quill.run {
      quote {
        query[NodeGraph].filter(ng => ng.networkId == lift(networkId)).filter(ng =>
          ng.nodeId == lift(nodeId) || ng.refId == lift(nodeId)
        )
      }
    }
    val nodeGraphMap = nodeGraph.map{v => 
      val id = if(nodeId == v.nodeId) v.refId else v.nodeId
      id -> v
    }.toMap
    
    val relativeNodes = quill.run {
      quote {
        query[Node].filter(n => lift(nodeGraphMap.keys.toList).contains(n.id) && n.status == lift(NodeStatus.Normal))
      }
    }
    
    
    (nodeGraphMap, relativeNodes)
  }
}
