#!/usr/bin/env bash

set -euox pipefail

cd "./$(dirname "$0")/.."

PATH="$HOME/.cargo/bin:$PATH"

cargo test --verbose
cargo fmt -- --write-mode diff --verbose ./tests/*.rs ./tests/**/*.rs ./src/*.rs ./src/*.rs
