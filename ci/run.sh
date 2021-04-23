#!/bin/bash

set -eo pipefail

DOCKER_RUST_IMG_NAME="reticle-rust"
DOCKER_RUST_APP_PATH="/usr/src/myapp"
DOCKER_RUST_REG_PATH="/usr/local/cargo/registry"
LOCAL_RUST_REG_PATH="$PWD/.cargo/registry"

# create a directory for cargo registry, so
# dependencies can be cached when running rust
# inside docker
mkdir -p $LOCAL_RUST_REG_PATH

docker build -t $DOCKER_RUST_IMG_NAME -f $PWD/docker/Dockerfile.rust .

function docker_rust {
    docker run --rm --pid=host \
        --user "$(id -u):$(id -g)" \
        -v "$PWD:$DOCKER_RUST_APP_PATH" \
        -v "$LOCAL_RUST_REG_PATH:$DOCKER_RUST_REG_PATH" \
        -w "$DOCKER_RUST_APP_PATH" \
        $DOCKER_RUST_IMG_NAME \
        $1
}

docker_rust "cargo fmt -- --check"
docker_rust "cargo clippy --all-targets --all-features -- -D warnings"
docker_rust "cargo test -p ir"
docker_rust "cargo test -p asm"
docker_rust "cargo test -p xir"
docker_rust "cargo test -p pat"
docker_rust "cargo test -p xim"
docker_rust "cargo test -p isel"
docker_rust "cargo test -p optimizer"
docker_rust "cargo test -p bler"
docker_rust "cargo test -p bline"
