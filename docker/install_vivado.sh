#!/bin/bash

VIVADO_BIN=$1

chmod +x $VIVADO_BIN

docker build -t "vivado" \
-f Dockerfile.vivado \
--build-arg USER_ID=$(id -u) \
--build-arg VIVADO_BIN=$VIVADO_BIN .
