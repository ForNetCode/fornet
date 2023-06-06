#!/usr/bin/env bash

# 1883(mqtt) 6060(http) 5363(grpc)
# docker rm -f mqtt
docker run -d --name mqtt --network=host -v $(pwd)/log:/var/log/rmqtt -v $(pwd)/config/rmqtt.toml:/app/rmqtt/rmqtt.toml -v $(pwd)/config/plugin:/app/rmqtt/plugin rmqtt/rmqtt:latest
# docker logs -f --tail 50 mqtt