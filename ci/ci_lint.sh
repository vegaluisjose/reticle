#!/bin/bash

set -eo pipefail

CARGO_CMD=$1

docker run --rm --pid=host --user "$(id -u)":"$(id -g)" -v "$PWD":/usr/src/myapp -w /usr/src/myapp "reticle-rust" cargo --version

# $CARGO_CMD cargo clippy --all-targets --all-features -- -D warnings
# $CARGO_CMD cargo fmt -- --check