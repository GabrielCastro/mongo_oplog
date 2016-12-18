#!/usr/bin/env bash

set -euox pipefail

cd "./$(dirname "$0")/.."

PATH="$HOME/.cargo/bin:$PATH"

build_flags=''

if [[ "$TRAVIS_RUST_VERSION" = 'nightly' ]] ; then
    cargo run install clippy
    rustup run nightly cargo clippy
fi

cargo build $build_flags
cargo test --verbose
cargo fmt -- --write-mode diff --verbose ./tests/*.rs ./tests/**/*.rs ./src/*.rs ./src/*.rs
