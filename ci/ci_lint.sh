#!/bin/bash

set -eo pipefail

DOCKER_RUST=$1

$DOCKER_RUST cargo clippy --all-targets --all-features -- -D warnings
$DOCKER_RUST cargo fmt -- --check