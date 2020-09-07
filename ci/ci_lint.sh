#!/bin/bash

set -eox pipefail

# eval "$DOCKER_RUST" cargo --version

$DOCKER_RUST cargo --version

# $CARGO_CMD cargo clippy --all-targets --all-features -- -D warnings
# $CARGO_CMD cargo fmt -- --check