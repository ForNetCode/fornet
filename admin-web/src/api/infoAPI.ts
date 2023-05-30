import http from "./http";
import {KeycloakConfig} from "keycloak-js";

type AuthTypeResponse = { type: 'ST' } | (KeycloakConfig & { type: 'Bearer' })
export function getAppInfo() {
    return http.get<AuthTypeResponse>('/info')
}
