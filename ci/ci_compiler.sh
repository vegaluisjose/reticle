#!/bin/bash

set -eo pipefail

WORKSPACE_DIR=/home/vivado/workspace

cargo run --bin regression
cd ci

docker run --rm --pid=host -v $PWD:$WORKSPACE_DIR -w $WORKSPACE_DIR "vivado" bash --login xsim.sh fsm
docker run --rm --pid=host -v $PWD:$WORKSPACE_DIR -w $WORKSPACE_DIR "vivado" bash --login xsim.sh register
docker run --rm --pid=host -v $PWD:$WORKSPACE_DIR -w $WORKSPACE_DIR "vivado" bash --login xsim.sh vadd_const

cd ../
