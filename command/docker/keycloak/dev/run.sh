#!/usr/bin/env bash

docker rm -f fornet-keycloak
# This is for linux, please change network=host if use others.
<<<<<<< Updated upstream
docker run -d --network=host --name=fornet-keycloak \
=======
docker run -d --network=host --name=fornet-keycloak\
>>>>>>> Stashed changes
    -e KC_DB_URL='jdbc:postgresql://127.0.0.1:5432/keycloak' \
    -e KC_DB_USERNAME=keycloak_user \
    -e KC_DB_PASSWORD='keycloak_db_password' \
    -e KC_HOSTNAME='keycloak-dev.fornetcode.com' \
    -e KC_HOSTNAME_STRICT_BACKCHANNEL=true \
    -e KC_PROXY=edge \
    -e KC_HOSTNAME_STRICT_HTTPS=false \
    -e KC_FEATURES=declarative-user-profile \
    -e KEYCLOAK_ADMIN=keycloak_admin \
    -e KEYCLOAK_ADMIN_PASSWORD=keycloak_password \
    -e KC_HTTP_PORT=8089 fornet-keycloak:dev start --optimized
docker logs -f --tail 200 fornet-keycloak
