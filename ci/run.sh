#!/bin/bash

set -eo pipefail

# command variable can be empty to run locally
DOCKER_RUST="docker run --rm \
--pid=host \
--user "$(id -u)":"$(id -g)" \
-v "$PWD":/usr/src/myapp \
-w /usr/src/myapp "reticle-rust""

source "ci/ci_lint.sh" "$DOCKER_RUST"
source "ci/ci_interpreter.sh" "$DOCKER_RUST"
source "ci/ci_compiler.sh" "$DOCKER_RUST"
source "ci/ci_vivado.sh"
