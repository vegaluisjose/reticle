#!/bin/bash

set -eo pipefail

# cargo fmt -- --check
# cargo clippy --all-targets --all-features -- -D warnings

DOCKER_RUST_IMG_NAME="reticle-rust"
DOCKER_RUST_APP_PATH="/usr/src/myapp"

# create a directory for cargo registry, so
# dependencies can be cached when running rust
# inside docker
mkdir -p $PWD/.cargo/registry

docker build -t $DOCKER_RUST_IMG_NAME -f $PWD/docker/Dockerfile.rust .

function docker_rust {
    docker run --rm --pid=host \
        --user "$(id -u):$(id -g)" \
        -v "$PWD:$DOCKER_RUST_APP_PATH" \
        -w "$DOCKER_RUST_APP_PATH" \
        $DOCKER_RUST_IMG_NAME \
        $1
}

docker_rust "cargo --version"
docker_rust "rustc --version"