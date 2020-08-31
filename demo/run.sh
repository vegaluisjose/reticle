#!/bin/bash

set -e
set -u
set -o pipefail

WORKSPACE_DIR=/home/vivado/workspace

cd ../
cargo run --bin demo
cd demo

docker run --rm --pid=host -v $PWD:$WORKSPACE_DIR -w $WORKSPACE_DIR "vivado" bash --login xsim.sh fsm
docker run --rm --pid=host -v $PWD:$WORKSPACE_DIR -w $WORKSPACE_DIR "vivado" bash --login xsim.sh register
