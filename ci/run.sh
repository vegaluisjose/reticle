#!/bin/bash

set -eo pipefail

cargo fmt -- --check
cargo clippy --all-targets --all-features -- -D warnings