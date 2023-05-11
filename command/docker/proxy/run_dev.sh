#!/usr/bin/env bash

#./build_docker.sh
#docker rm -f nginx
# .run
# 192.168.31.146 is localhost ip

#HOST_IP=$1
#docker run -d --name=nginx --add-host=proxy.dev:$HOST_IP -p 80:80 fornet_nginx:dev

#docker rm -f nginx && \
docker run -d --name=nginx --network=host  --add-host=proxy.dev:127.0.0.1 -v $(pwd)/nginx.conf:/etc/nginx/nginx.conf nginx:alpine && \
docker logs -f --tail 50 nginx