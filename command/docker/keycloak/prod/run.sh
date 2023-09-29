#!/usr/bin/env bash

#docker rm -f fornet-keycloak
export KC_PG_DB='keycloak'
export KC_DB_USERNAME=''
export KC_DB_PASSWORD=''
# This is for linux, please change network=host if use others.
docker run -d --network=host --name=fornet-keycloak\
    -e KC_DB_URL='jdbc:postgresql://127.0.0.1:5432/'${KC_PG_DB} \
    -e KC_DB_USERNAME=${KC_DB_USERNAME} \
    -e KC_DB_PASSWORD=${KC_DB_PASSWORD} \
    -e KC_HOSTNAME='sso.fornetcode.com'\
    -e KC_HOSTNAME_STRICT_BACKCHANNEL=true \
    -e KC_PROXY=edge\
    -e KC_HOSTNAME_STRICT_HTTPS=false \
    -e KC_FEATURES=declarative-user-profile\
    -e KEYCLOAK_ADMIN=keycloak_admin\
    -e KEYCLOAK_ADMIN_PASSWORD=keycloak_password\
    -e KC_HTTP_PORT=8089 fornet-keycloak:prod start --optimized --hostname-admin=localhost
docker logs -f --tail 200 fornet-keycloak
