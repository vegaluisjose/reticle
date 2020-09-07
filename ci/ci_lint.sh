#!/bin/bash

set -eo pipefail

CARGO_CMD=$1

$CARGO_CMD ls

# $CARGO_CMD cargo clippy --all-targets --all-features -- -D warnings
# $CARGO_CMD cargo fmt -- --check