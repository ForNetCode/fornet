import http, {CreatedSuccess} from "./http";


export enum NodeStatus {
    Waiting, Connected, Normal, Forbid, Delete
}

export enum NodeType {
    Client, Relay
}

export interface Node {
    id: number,
    nodeType: NodeType,
    status: NodeStatus,
    setting: NodeSetting,
    networkId: number,
    name: string,
}

export interface NodeSetting {
    port?: number,
    keepAlive?: number,
    mtu?: number,
    endpoint?: string,
    dns?: number,
    postUp?:string,
    postDown?: string,
}


export interface UpdateNode {
    name: string,
    setting: NodeSetting,
}

export function getNodeList(networkId: number, page: number = 1, pageSize: number = 10) {
    return http.get<Node[]>(`/node/${networkId}`, {
        params: {
            page,
            pageSize
        }
    })
}

export interface CreateNode {
    name: string,
    ip?: string,
    nodeType: NodeType,
    setting: NodeSetting,
}

export function createNode(networkId: number, data: CreateNode) {
    return http.post<CreatedSuccess>(`/node/${networkId}`, data)
}

export function getNode(networkId: number, nodeId: number) {
    return http.get<Node>(`/node/${networkId}/${nodeId}`).then(r => r.data)
}

export function updateNode(networkId: number, nodeId: number, data: UpdateNode) {
    return http.put(`/node/${networkId}/${nodeId}`, data)
}

export function getNodeActiveCode(networkId: number, nodeId: number) {
    return http.get<string>(`/node/${networkId}/${nodeId}/active_code`).then(r => r.data)
}

export function updateNodeStatus(networkId: number, nodeId: number, status: NodeStatus.Forbid | NodeStatus.Normal) {
    return http.put(`/node/${networkId}/${nodeId}/status`, {status})
}
