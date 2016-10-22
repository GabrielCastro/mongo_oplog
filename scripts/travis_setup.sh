#!/usr/bin/env bash

set -euox pipefail

cd "./$(dirname "$0")/.."

PATH="$HOME/.cargo/bin:$PATH"

exec cargo install --verbose rustfmt
