#!/usr/bin/env bash

IP=$(ifconfig -l | xargs -n1 ipconfig getifaddr)

docker run \
-e APP_ENVIRONMENT="staging" \
-e ASSETS_DIR="./assets" \
-e VITE_DIR="./frontend/dist" \
-e BASE_PATH="./" \
-e PORT="8000" \
-e VITE_ENVIRONMENT="staging" \
-e DATABASE_URL="postgres://postgres:password@${IP}:5555/brain-wars" \
-p 8000:8000 \
brain-wars

