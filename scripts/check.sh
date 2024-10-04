#!/usr/bin/env bash
export APP_ENVIRONMENT=local
export DATABASE_URL="postgres://postgres:password@localhost:5555/brain-wars"
docker rm "$(docker stop "$(docker ps -a -q --filter ancestor=postgres:15.3-alpine3.18 --format="{{.ID}}")")"
docker volume prune --force
./scripts/init_db.sh
if [ "$?" -ne 0 ]
then
    printf "\ninit_db.sh failed\n"
    exit 1
fi

#source ./scripts/enable_coverage.sh

cargo check
if [ "$?" -ne 0 ]
then
    printf "\ncargo check failed\n"
    exit 1
fi