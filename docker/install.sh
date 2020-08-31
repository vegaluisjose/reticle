#!/bin/bash

VIVADO_BIN=Xilinx_Unified_2020.1_0602_1208_Lin64.bin

chmod +x $VIVADO_BIN

docker build -t "vivado" \
-f Dockerfile.vivado \
--build-arg USER_ID=$(id -u) \
--build-arg VIVADO_BIN=$VIVADO_BIN .
