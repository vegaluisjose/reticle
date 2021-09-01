#!/bin/bash
OUTLOG=output.txt

cargo run --example xilinx_inverter

source /tools/Xilinx/Vivado/2020.1/settings64.sh
time make all > $OUTLOG

#source /tools/Xilinx/Vivado/2020.2/settings64.sh
#time make all >> $OUTLOG

#source /tools/Xilinx/Vivado/2021.1/settings64.sh
#time make all >> $OUTLOG