#!/bin/bash

OUT_DIR=/home/vivado/output
WORKSPACE_DIR=/home/vivado/workspace

source /tools/Xilinx/Vivado/2020.1/settings64.sh

mkdir -p $OUT_DIR
cd $OUT_DIR

xvlog --sv --incr --relax $XILINX_VIVADO/data/verilog/src/glbl.v $WORKSPACE_DIR/${1}.v $WORKSPACE_DIR/test_${1}.v
xelab -L unisims_ver -R --timescale 1ps/1ps -O3 glbl test_${1}
