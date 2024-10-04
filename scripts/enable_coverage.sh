#!/usr/bin/env bash
cargo clean
export LLVM_PROFILE_FILE='coverage/cargo-test-%p-%m.profraw'
export RUSTFLAGS='-Cinstrument-coverage'
export CARGO_INCREMENTAL=0
mkdir -p coverage/ && rm -r coverage/*