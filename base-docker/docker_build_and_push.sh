#!/bin/bash

platform=""
architecture="$(arch)"

docker buildx use cloud-ohaddahan-cloud-builder

if [ "${architecture}" = "arm64" ] ; then
  platform="linux/arm64"
#  docker buildx create --use --name mybuilder --node node-amd64
fi
if [ "${architecture}" = "x86_64" ] ; then
  platform="linux/amd64"
#  docker buildx create --use --name mybuilder --node node-amd64
fi

architecture="x86_x64"
platform="linux/amd64"

docker buildx build \
--builder cloud-ohaddahan-cloud-builder \
--ulimit nofile=1024000:1024000 \
--tag ohaddahan/brain-wars-base:${architecture} \
--platform "${platform}" \
--file Dockerfile . --push