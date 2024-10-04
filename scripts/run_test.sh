#!/usr/bin/env bash
set -x
_PWD="$(pwd)"
ROOT="$(git rev-parse --show-toplevel)"
cd "${ROOT}/backend" || exit
set +x
cargo install cargo-nextest --locked
export APP_ENVIRONMENT=test
export WALLET_MNEMONIC="${ROOT}/backend/fixtures/test_keypair.json"
#export TEST_LOG=1
#export RUST_BACKTRACE=1
export DATABASE_URL="postgres://postgres:password@localhost:5555/brain-wars"
docker rm "$(docker stop "$(docker ps -a -q --filter ancestor=postgres:15.3-alpine3.18 --format="{{.ID}}")")"
docker volume prune --force
"${ROOT}/scripts/init_db.sh"
if [ "$?" -ne 0 ]
then
    printf "\nbuild.sh failed\n"
    exit 1
fi
# Remove possible existing coverages

#source ./scripts/enable_coverage.sh
#TEST_MNEMONIC=$(cat ./fixtures/eth_wallet.mnemonic)

export BPF_OUT_DIR="${ROOT}/target/deploy/"
#npx ganache --port 7654 --wallet.mnemonic "${TEST_MNEMONIC}" &> /dev//null &
#export ganache=$!

#sleep 1
#CONTRACT_ADDRESS=$(cd fixtures && npx truffle migrate --reset | \grep 'contract address' | sed -e 's/.*x/0x/')
#export CONTRACT_ADDRESS

#function cleanup()
#{
#  echo "Killing $ganache"
#  pkill -P $ganache
#}

#trap cleanup SIGINT EXIT

#cargo nextest run create_game --features my-test
cargo test --features my-test -- --nocapture | bunyan
#sleep 2
#cat "${ROOT}/backend/logs/*.log" | bunyan > backend/logs/bunyan.log
#cargo nextest run --nocapture | bunyan
if [ "$?" -ne 0 ]
then
    printf "\cargo test failed\n"
    cd "${_PWD}" || exit
    exit 1
fi
#grcov . --binary-path ./target/debug/deps/ -s . -t html --branch --ignore-not-existing --ignore '../*' --ignore "/*" -o target/coverage/ &
##grcov . --binary-path ./target/debug/deps/ -s . -t lcov --branch --ignore-not-existing --ignore '../*' --ignore "/*" -o target/coverage/lcov.info &
#grcov . --binary-path ./target/debug/deps/ -s . -t markdown --branch --ignore-not-existing --ignore '../*' --ignore "/*" -o target/coverage/markdown.md
#cat target/coverage/markdown.md
#open -a "Google Chrome"  target/coverage/index.html &