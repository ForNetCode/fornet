#!/usr/bin/env bash

if [ -d ../backend/src/main/webapp ];then
  rm -fr ../backend/src/main/webapp/*
fi
npm run build:prod && cp -r build/* ../backend/src/main/webapp/