#!/usr/bin/env bash
./scripts/build.sh
if [ "$?" -ne 0 ]
then
    printf "\nbuild.sh failed\n"
    exit 1
fi
target/debug/backend | bunyan