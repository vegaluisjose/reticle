#!/bin/bash

set -eo pipefail

# cargo fmt -- --check
# cargo clippy --all-targets --all-features -- -D warnings

# create a directory for cargo registry, so
# dependencies can be cached when running rust
# inside docker
mkdir -p $PWD/.cargo/registry

source $PWD/docker/install_rust.sh