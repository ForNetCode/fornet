#!/usr/bin/env bash
# ./docker_build.sh 0.0.3

if [ $# -eq 0 ]
  then
    echo "No version set"
    exit -1
fi
VERSION=$1

echo "begin to build fornet command client $VERSION"

BASE_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && cd ../../../ && pwd)

cd $BASE_DIR

DOCKER_BUILDKIT=1 docker build -t=fornet:$VERSION -f $BASE_DIR/command/docker/client/client.Dockerfile .