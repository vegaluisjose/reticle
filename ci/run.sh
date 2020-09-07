#!/bin/bash

set -eo pipefail

# command variable can be empty to run locally
CARGO_CMD=`docker run --rm \
--pid=host \
--user "$(id -u)":"$(id -g)" \
-v "$PWD":/usr/src/myapp \
-w /usr/src/myapp "reticle-rust"`

function test_ci {
    res=source "$1" "$2"
    if [ $res ] ; then
       echo -e "\033[01;31m[Fail] $1"
    else
       echo -e "\033[01;32m[Pass] $1"
    fi
}

test_ci "ci/ci_lint.sh" $CARGO_CMD
test_ci "ci/ci_interpreter.sh" $CARGO_CMD
