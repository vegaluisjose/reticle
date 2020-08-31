#!/bin/bash

/tmp/wi/xsetup -b AuthTokenGen
/tmp/wi/xsetup -b ConfigGen

WORKSPACE_DIR=/home/vivado/workspace

cp /home/vivado/.Xilinx/wi_authentication_key $WORKSPACE_DIR
cp /home/vivado/.Xilinx/install_config.txt $WORKSPACE_DIR
