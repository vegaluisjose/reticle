#!/bin/bash

set -eo pipefail

# command variable can be empty to run locally
CARGO_CMD=`docker run --rm \
--pid=host \
--user "$(id -u)":"$(id -g)" \
-v "$PWD":/usr/src/myapp \
-w /usr/src/myapp "reticle-rust"`


source "ci/ci_lint.sh" $CARGO_CMD
source "ci/ci_interpreter.sh" $CARGO_CMD
source "ci/ci_compiler.sh" $CARGO_CMD
source "ci/ci_vivado.sh"
