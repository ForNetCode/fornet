#!/usr/bin/env bash

# this runs in Docker to build 

mkdir /protoc && cd /protoc

architecture=$(uname -m)

if [ $architecture == aarch64 ];
then
    architecture='aarch_64'
fi

wget https://github.com/protocolbuffers/protobuf/releases/download/v21.9/protoc-21.9-linux-$architecture.zip && unzip protoc-21.9-linux-$architecture.zip && cp bin/* /usr/bin/

cd ../

#cp -r /protoc/include/* /source/protobuf/