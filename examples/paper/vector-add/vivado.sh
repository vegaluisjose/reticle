#!/bin/bash

VIVADO_TCL=$1
FILENAME=$2
OUT_DIR=$3
NAME=$4

source /tools/Xilinx/Vivado/2020.1/settings64.sh
vivado -mode batch -source $VIVADO_TCL -tclargs $FILENAME $OUT_DIR $NAME
