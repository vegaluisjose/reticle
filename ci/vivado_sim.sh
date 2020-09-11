#!/bin/bash

TEST_NAME=$1
TEST_FILE=$2
DUT_FILE=$3
OUT_DIR=$4

source /tools/Xilinx/Vivado/2020.1/settings64.sh

mkdir -p $OUT_DIR
cd $OUT_DIR

xvlog $XILINX_VIVADO/data/verilog/src/glbl.v $TEST_FILE $DUT_FILE
xelab -L unisims_ver -R --timescale 1ps/1ps -O3 glbl $TEST_NAME
