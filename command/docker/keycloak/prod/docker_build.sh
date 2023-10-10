#!/usr/bin/env bash
if [ -d ./fornet ];then
  rm -fr fornet
fi
cp -r ../../../../third/keycloak-theme/fornet .
DOCKER_BUILDKIT=1 docker build . -t=fornet-keycloak:prod -f prod.Dockerfile
