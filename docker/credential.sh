#!/bin/bash

VIVADO_BIN=Xilinx_Unified_2020.1_0602_1208_Lin64.bin

chmod +x $VIVADO_BIN

docker build -t "vivado-credential" \
-f Dockerfile.credential \
--build-arg USER_ID=$(id -u) \
--build-arg VIVADO_BIN=$VIVADO_BIN .

docker run --rm -i -t -v "$(pwd)":/home/vivado/workspace "vivado-credential" /bin/bash /tmp/auth.sh
