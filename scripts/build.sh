#!/usr/bin/env bash
set -x
_PWD="$(pwd)"
ROOT="$(git rev-parse --show-toplevel)"
cd "${ROOT}/backend" || exit
set +x

export APP_ENVIRONMENT=local
if [ -n "${DATABASE_URL+1}" ]; then
  export DATABASE_URL="postgres://postgres:password@localhost:5555/brain-wars"
fi
#docker system prune --all --force
#docker rm --force --volumes "$(docker stop "$(docker ps -a -q --filter ancestor=postgres:15.3-alpine3.18 --format="{{.ID}}")")"
#DOCKERS="$(docker ps -a -q --filter ancestor=postgres:15.3-alpine3.18 --format="{{.ID}}")"
#docker rm --force --volumes "$(docker ps -a -q --filter ancestor=postgres:15.3-alpine3.18 --format="{{.ID}}")"
#docker rm --force --volumes "$(docker ps -a -q --filter ancestor=postgres:15.3-alpine3.18 --format="{{.ID}}")"
#docker volume prune --force
"${ROOT}/scripts/init_db.sh"
if [ "$?" -ne 0 ]
then
    printf "\n init_db.sh failed\n"
    cd "${_PWD}" || exit
    exit 1
fi

#source ./scripts/enable_coverage.sh

cargo build
if [ "$?" -ne 0 ]
then
    printf "\n cargo build failed\n"
    cd "${_PWD}" || exit
    exit 1
fi
export RUST_BACKTRACE=1
typeshare . --lang=typescript --output-file="${ROOT}/client/brain-war-client/src/helpers/apiTypes.ts"
if [ "$?" -ne 0 ]
then
    printf "\n typeshare failed\n"
    cd "${_PWD}" || exit
    exit 1
fi
cd "${ROOT}"
cd "client/brain-war-client" && npm install && npm run build
if [ "$?" -ne 0 ]
then
   printf "\n npm run build failed\n"
   exit 1
fi
cd "${ROOT}"
cp -fr "client/brain-war-client/dist" backend
