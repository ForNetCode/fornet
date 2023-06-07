import http, {CreatedSuccess, ID, Page} from "./http";

export enum NetworkProtocol {
    TCP, UDP
}

export interface NetworkSetting {
    mtu: number,
    keepAlive: number,
    dns?: string,
    port: number,
    protocol: NetworkProtocol,
}

export interface Network {
    id: ID,
    name: string,
    setting: NetworkSetting,
    createdAt: string,
    updatedAt: string,
    addressRange: string,

}

export function networkList(name: string | null, page: number = 1, pageSize: number = 10) {
    return http.get<Page<Network>>('/network', {
        params: {
            name,
            page,
            pageSize
        }
    }).then(r => r.data)
}

export interface CreateNetwork {
    name: string,
    addressRange: string,
    protocol: NetworkProtocol,
}

export function createNetwork(data: CreateNetwork) {
    return http.post<CreatedSuccess>('/network', data)
}

export function updateNetwork(id: number, data: Network) {
    return http.put<Network>(`/network/${id}`, data)
}

export function getNetwork(id: ID) {
    return http.get<Network>(`/network/${id}`).then(r => r.data)
}

export function getNetworkInviteCode(networkId: ID) {
    return http.get<string>(`/network/${networkId}/invite_code`).then(r => r.data)
}


export function deleteNetwork(id:ID) {
    return http.delete(`/network/${id}`)
}
