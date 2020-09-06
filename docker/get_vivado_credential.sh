#!/bin/bash

VIVADO_BIN=$1
WORKSPACE_DIR=$2

chmod +x $VIVADO_BIN

docker build -t "vivado-credential" \
-f Dockerfile.credential \
--build-arg USER_ID=$(id -u) \
--build-arg VIVADO_BIN=$VIVADO_BIN .

docker run --rm -i -t \
-v $PWD:$WORKSPACE_DIR \
-w $WORKSPACE_DIR \
"vivado-credential" \
bash --login auth.sh $WORKSPACE_DIR
