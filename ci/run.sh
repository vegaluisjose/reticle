#!/bin/bash

set -eo pipefail

# cargo fmt -- --check
# cargo clippy --all-targets --all-features -- -D warnings

DOCKER_RETICLE_IMG_NAME="reticle-rust"

# create a directory for cargo registry, so
# dependencies can be cached when running rust
# inside docker
mkdir -p $PWD/.cargo/registry

docker build -t $DOCKER_RETICLE_IMG_NAME -f $PWD/docker/Dockerfile.rust .

docker run --rm --pid=host \
    --user "$(id -u):$(id -g)"
    $DOCKER_RETICLE_IMG_NAME
    "ls"