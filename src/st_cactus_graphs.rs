use std::collections::HashSet;
use std::collections::HashMap;

//TODO: check types of collections

struct CactusNode {
    head : Option<Box<CactusEdgeEnd>>,
    tail : Option<Box<CactusEdgeEnd>>,
    nodeObject : u64
}

struct CactusEdgeEnd {
    otherEdgeEnd : Box<CactusEdgeEnd>,
    node : CactusNode,
    nEdgeEnd : Box<CactusEdgeEnd>,
    endObject : u64,
    link : Box<CactusEdgeEnd>,
    linkOrientaton : bool,
    isChainEnd : bool
}

struct CactusGraph {
    objectToNodeHash : HashMap<u64, u64>
}

struct CactusNodeEdgeEndIt {
    edgeEnd : CactusEdgeEnd
}

struct Snarl {
    edgeEnd1 : CactusEdgeEnd,
    edgeEnd2 : CactusEdgeEnd,
    chains : Vec<u64>,
    unarySnarls : Vec<u64>,
    parentCount : u64
}

struct SnarlDecomposition {
    topLevelChains : Vec<u64>,
    topLevelUnarySnarls : Vec<u64>
}

struct BridgeNode {
    connectedNodes : Vec<BridgeNode>,
    bridgeEnds : Vec<CactusEdgeEnd>,
    cactusNodes : HashSet<CactusNode>,
    connectedCactusNodes : HashSet<CactusNode>
}

struct BridgeGraph {
    bridgeNodes : Vec<BridgeNode>
}

fn CactusNodeContruct(graph : &CactusGraph, nodeObject : u64) -> CactusNode {
    let node = CactusNode {
        head : None,
        tail : None,
        nodeObject : nodeObject
    };

    node   
}

fn CactusNode_getObject(node : &CactusNode) -> u64 {
    node.nodeObject
}

fn CactusNode_getEdgeEndIt(node : &CactusNode) -> Option<Box<CactusEdgeEnd>> {
    node.head
}

fn CactusNodegetEdgeEndIt_getNext(it : &CactusNodeEdgeEndIt) -> CactusEdgeEnd {
    let edgeEnd = it.edgeEnd;
    //if edgeEnd.isNone() {
    //}
    edgeEnd
}

fn CactusNode_getFirstEdgeEnd(node : &CactusNode) -> Option<Box<CactusEdgeEnd>> {
    node.head
}

fn CactusGraph_getNodeNumber(graph : &CactusGraph) -> usize {
    graph.objectToNodeHash.len()
}

// TODO: Private node functions

// Edge functions

fn connectUpEdgeEnd(edgeEnd : &CactusEdgeEnd, node : &CactusNode, otherEdgeEnd : &CactusEdgeEnd, u64 endObject) {
    edgeEnd.node = *node;
    edgeEnd.otherEdgeEnd = otherEdgeEnd;
    edgeEnd.endObject = endObject;

    if node.head.is_none() {
        node.head = edgeEnd;
    } else {
        node.tail.nEdgeEnd = edgeEnd;
    }

    node.tail = edgeEnd;
}

fn CactusEdgeEnd_construct(graph : &CactusGraph, node1 : CactusNode, node2 : CactusNode, edgeEndObject1 : u64, edgeEndObject2 : u64) -> CactusEdgeEnd {
    
    let edgeEnd1 : CactusEdgeEnd;
    let edgeEnd2 : CactusEdgeEnd;

    connectUpEdgeEnd(&edgeEnd1, &node1, &edgeEnd2, edgeEndObject1);
    connectUpEdgeEnd(&edgeEnd2, &node2, &edgeEnd1, edgeEndObject2);
    
    edgeEnd1
}

fn CactusEdgeEnd_getObject(edgeEnd : &CactusEdgeEnd) -> u64 {
    edgeEnd.endObject
}

fn CactusEdgeEnd_getNode(edgeEnd : &CactusEdgeEnd) -> CactusNode {
    edgeEnd.node
}

fn CactusEdgeEnd_getOtherNode(edgeEnd : &CactusEdgeEnd) -> CactusNode {
    edgeEnd.otherEdgeEnd.node
}

fn CactusEdgeEnd_getOtherEdgeEnd(edgeEnd : &CactusEdgeEnd) -> Box<CactusEdgeEnd> {
    edgeEnd.otherEdgeEnd
}

fn CactusEdgeEnd_getLink(edgeEnd : &CactusEdgeEnd) -> Box<CactusEdgeEnd> {
    edgeEnd.link
}

fn CactusEdgeEnd_getLinkOrientation(edgeEnd : &CactusEdgeEnd) -> bool {
    edgeEnd.linkOrientaton
}

fn CactusEdgeEnd_isChainEnd(edgeEnd : &CactusEdgeEnd) -> bool {
    edgeEnd.isChainEnd
}

fn CactusEdgeEnd_getNextEdgeEnd(edgeEnd : &CactusEdgeEnd) -> Box<CactusEdgeEnd> {
    edgeEnd.nEdgeEnd
}

// Private edge functions

fn CactusEdgeEnd_setLink(edgeEnd : &CactusEdgeEnd, otherEdgeEnd : &CactusEdgeEnd) {
    edgeEnd.link = otherEdgeEnd;
}

fn CactusEdgeEnd_setLinkOrientation(edgeEnd : &CactusEdgeEnd, orientation : bool) {
    edgeEnd.linkOrientaton = orientation;
}

fn CactusEdgeEnd_setIsChainEnd(edgeEnd : &CactusEdgeEnd, isChainEnd : bool) {
    edgeEnd.isChainEnd = isChainEnd;
}

// TODO: Graph functions














