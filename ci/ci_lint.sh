#!/bin/bash

set -eox pipefail

DOCKER_RUST=$(docker run --rm --pid=host --user "$(id -u)":"$(id -g)" -v "$PWD":/usr/src/myapp -w /usr/src/myapp "reticle-rust")

# eval "$DOCKER_RUST" cargo --version

CMD=$($DOCKER_RUST cargo --version)

echo $CMD

# $CARGO_CMD cargo clippy --all-targets --all-features -- -D warnings
# $CARGO_CMD cargo fmt -- --check