#!/usr/bin/env bash

if [ $# -eq 0 ]
  then
    echo "No version set"
    exit -1
fi

VERSION=$1

echo "begin to build backend $VERSION"

BASE_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && cd ../../../ && pwd )
SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd)

cd $BASE_DIR/command/docker/backend

if [ -d $SCRIPT_DIR/app ];then
   rm -fr $SCRIPT_DIR/app
fi


#if [ -d $BASE_DIR/backend/src/main/webapp ];then
#   rm -fr $BASE_DIR/backend/src/main/webapp
#fi
if [ -d $SCRIPT_DIR/web ];then
   rm -fr $SCRIPT_DIR/web
fi


cd $BASE_DIR/admin-web && npm run build:prod

if [ $? -eq 0 ];then
   echo "build admin-web success"
   cp -r $BASE_DIR/admin-web/build/ $SCRIPT_DIR/web
else
   echo "build admin-web failure"
   exit -1
fi


cd $BASE_DIR/backend && sbt universal:packageBin

if [ $? -eq 0 ];then
   echo "build backend success"
else
   echo "build backend failure"
   exit -1
fi

cp $BASE_DIR/backend/target/universal/app-$VERSION.zip $SCRIPT_DIR/app.zip


#cp -r $BASE_DIR/admin-web/build  $SCRIPT_DIR/web
cd $SCRIPT_DIR && unzip app.zip &&  mv app-$VERSION app

DOCKER_BUILDKIT=1 docker build . -t=fornet-backend:$VERSION

if [ $? -eq 0 ];then
   echo "build fornet-backend image successfully!"
fi