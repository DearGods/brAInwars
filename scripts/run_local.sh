#!/usr/bin/env bash
set -x
_PWD="$(pwd)"
ROOT="$(git rev-parse --show-toplevel)"
cd "${ROOT}/backend" || exit
set +x
export APP_ENVIRONMENT=local
export FULL_BASE_PATH="${ROOT}/backend"
export VITE_ENVIRONMENT="${APP_ENVIRONMENT}"
export WALLET_MNEMONIC="${FULL_BASE_PATH}/fixtures/crank.json"
export DATABASE_URL="postgres://postgres:password@localhost:5555/brain-wars"
"${ROOT}/scripts/init_db.sh"
if [ "$?" -ne 0 ]
then
    printf "\n init_db.sh failed\n"
    exit 1
fi
"${ROOT}/scripts/build.sh"
if [ "$?" -ne 0 ]
then
    printf "\n build.sh failed\n"
    exit 1
fi
"${ROOT}/target/debug/backend" &
 #| bunyan &
export backend=$!

# cd "${ROOT}/client/brain-war-client" && npm run serve &
# export frontend=$!

function cleanup()
{
  echo "Killing $backend and $frontend"
  kill $backend
  kill $frontend
}

trap cleanup SIGINT EXIT

wait $backend
wait $frontend