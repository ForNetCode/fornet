#!/usr/bin/env bash

# 8883(mqtt) 6060(http) 5363(grpc)
# docker rm -f mqtt-cert
docker run -d --name mqtt-cert --network=host -v $(pwd)/log:/var/log/rmqtt -v $(pwd)/config/cert:/cert -v $(pwd)/config/rmqtt_ssl.toml:/app/rmqtt/rmqtt.toml -v $(pwd)/config/plugin:/app/rmqtt/plugin rmqtt/rmqtt:0.2.16
# docker logs -f --tail 50 mqtt-cert