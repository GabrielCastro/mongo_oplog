#!/usr/bin/env bash

set -euox pipefail

cd "./$(dirname "$0")/.."

PATH="$HOME/.cargo/bin:$PATH"

if [[ ! -x "$HOME/.cargo/bin/rustfmt" ]] ; then
    exec cargo install --verbose rustfmt;
fi
