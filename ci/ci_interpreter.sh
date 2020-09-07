#!/bin/bash

set -eo pipefail

DOCKER_RUST=$1

$DOCKER_RUST cargo test
