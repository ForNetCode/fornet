#!/usr/bin/env bash


# This is test for backend image

#docker rm -f nginx && \
docker run -d --name=nginx-test --network=host  --add-host=proxy.dev:127.0.0.1 -v $(pwd)/nginx.test.conf:/etc/nginx/nginx.conf nginx:alpine && \
docker logs -f --tail 50 nginx-test