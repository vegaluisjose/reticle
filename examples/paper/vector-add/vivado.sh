#!/bin/bash

#export TESTS_DIR=$1
#export VERILOG_DIR=$2
#OUT_DIR=$3
#
#source /tools/Xilinx/Vivado/2020.1/settings64.sh
#
#mkdir -p $OUT_DIR
#cd $OUT_DIR
#
#xvlog -f $TESTS_DIR/files.tcl
#xelab -L unisims_ver -R --timescale 1ps/1ps -O3 glbl test_all
#
#cd ..
#rm -rf $OUT_DIR

source /tools/Xilinx/Vivado/2020.1/settings64.sh
vivado -version
