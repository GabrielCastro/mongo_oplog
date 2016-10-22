#!/usr/bin/env bash

set -euox pipefail

cd "./$(dirname "$0")/.."

cargo test --verbose
cargo fmt -- --write-mode diff --verbose ./test/*.rs ./test/**/*.rs ./src/*.rs ./src/**/*.rs
