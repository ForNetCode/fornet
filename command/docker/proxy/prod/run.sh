#!/usr/bin/env bash


# docker kill --signal=HUP proxy


docker run -d --network=host --name=proxy \
           --env CERTBOT_EMAIL=tech@fornetcode.com \
           -v $(pwd)/nginx_secrets:/etc/letsencrypt \
           -v $(pwd)/nginx.conf:/etc/nginx/user_conf.d/nginx.conf:ro \
           --name nginx-certbot jonasal/nginx-certbot:5.0-alpine