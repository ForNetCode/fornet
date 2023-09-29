#!/usr/bin/env bash

DOCKER_BUILDKIT=1 docker build . -t=fornet-keycloak:dev -f dev.Dockerfile