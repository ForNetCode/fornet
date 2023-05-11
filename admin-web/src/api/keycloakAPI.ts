import Keycloak, {KeycloakConfig} from "keycloak-js";
import {savePersistenceToken} from "./authAPI";
import {updateHttpToken} from "./http";


let keycloak: Keycloak

let intervalId: NodeJS.Timer | null = null

export async function initKeycloak(config: KeycloakConfig) {

    keycloak = new Keycloak(config)
    let auth = await keycloak.init({
        onLoad: 'login-required'
    })
    if (auth) {
        let token = `Bearer ${keycloak.token}`
        savePersistenceToken(token)
        updateHttpToken(token)
    }

    if (intervalId != null) {
        console.log('clear interval id')
        clearInterval(intervalId)
    }

    intervalId = setInterval(() => {
        keycloak.updateToken(70).then((refreshed) => {
            if (refreshed) {
                let token = `Bearer ${keycloak.token}`
                savePersistenceToken(token)
                updateHttpToken(token)
                console.log('refreshed token')
            }
        })
    }, 1000 * 50)


}

