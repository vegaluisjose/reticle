#!/bin/bash

NAME=$1
WORKSPACE_DIR=$2
OUT_DIR=$3

source /tools/Xilinx/Vivado/2020.1/settings64.sh

mkdir -p $OUT_DIR
cd $OUT_DIR

xvlog --sv --incr --relax $XILINX_VIVADO/data/verilog/src/glbl.v $WORKSPACE_DIR/${NAME}.v $WORKSPACE_DIR/test_${NAME}.v
xelab -L unisims_ver -R --timescale 1ps/1ps -O3 glbl test_${NAME}
