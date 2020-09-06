#!/bin/bash

VIVADO_BIN=Xilinx_Unified_2020.1_0602_1208_Lin64.bin
WORKSPACE_DIR=/home/vivado/workspace

if [ ! -f "$VIVADO_BIN" ]; then
    echo "$VIVADO_BIN was not found in docker directory."
    exit 1
fi

source get_vivado_credential.sh $VIVADO_BIN $WORKSPACE_DIR
#source install_vivado.sh $VIVADO_BIN
source install_rust.sh
