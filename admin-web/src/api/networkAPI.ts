import http, {CreatedSuccess, Page} from "./http";


export interface NetworkSetting {
    mtu: number,
    keepAlive: number,
    dns?: string,
    port: number,
}

export interface Network {
    id: number,
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
}

export function createNetwork(data: CreateNetwork) {
    return http.post<CreatedSuccess>('/network', data)
}

export function updateNetwork(id: number, data: Network) {
    return http.put<Network>(`/network/${id}`, data)
}

export function getNetwork(id: number) {
    return http.get<Network>(`/network/${id}`).then(r => r.data)
}

export function getNetworkInviteCode(networkId: number) {
    return http.get<string>(`/network/${networkId}/invite_code`).then(r => r.data)
}


export function deleteNetwork(id:number) {
    return http.delete(`/network/${id}`)
}
