#!/usr/bin/env bash

#docker rm -f fornet-keycloak
#docker run -d  --name postgres -p 5432:5432 \
#-e POSTGRES_PASSWORD=postgres \
#-e POSTGRES_USER=postgres \
#-v $PWD/pg2:/var/lib/postgresql/data \
#postgres:15

# This is for linux, please change network=host if use others.

export KC_DB_URL='jdbc:postgresql://127.0.0.1:5432/keycloak'
export KC_DB_USERNAME='postgres'
export KC_DB_PASSWORD='postgres'
export KEYCLOAK_ADMIN='keycloak_admin'
export KEYCLOAK_ADMIN_PASSWORD='keycloak_password'
docker run -d --netowkr=host --name=keycloak \
    -e KC_DB_URL=${KC_DB_URL} \
    -e KC_DB_USERNAME=${KC_DB_USERNAME} \
    -e KC_DB_PASSWORD=${KC_DB_PASSWORD} \
    -e KC_HOSTNAME='sso.fornetcode.com' \
    -e KC_HOSTNAME_STRICT_BACKCHANNEL=true \
    -e KC_PROXY=edge \
    -e KC_HOSTNAME_STRICT_HTTPS=false \
    -e KC_FEATURES=declarative-user-profile \
    -e KEYCLOAK_ADMIN=${KEYCLOAK_ADMIN} \
    -e KEYCLOAK_ADMIN_PASSWORD=${KEYCLOAK_ADMIN_PASSWORD} \
    -e KC_HTTP_PORT=8089 fornet-keycloak:prod start --optimized --hostname-admin=localhost
docker logs -f --tail 200 fornet-keycloak
