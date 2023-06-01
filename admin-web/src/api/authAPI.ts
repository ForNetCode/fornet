import http from "./http";

export function checkSampleTokenCorrect(token: string) {
    return http.post('/auth/st/check', {
        token,
    })
}


export function getSSOInviteCode(networkId:number):Promise<string> {
    // @ts-ignore
    return http.get<string>(`/auth/oauth/${networkId}/device_code`, {
        // @ts-ignore
        disableDefaultErrorHandler: true,
    }).then((r) => r.data)
}

const TokenKey = "TOKEN_KEY"

export function getPersistenceToken():string|null {
    return localStorage.getItem(TokenKey)
}

export function savePersistenceToken(token: string) {
    return localStorage.setItem(TokenKey, token)
}

export function clearPersistenceToken() {
    return localStorage.removeItem(TokenKey)
}


