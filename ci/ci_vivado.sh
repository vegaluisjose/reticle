#!/bin/bash

set -eo pipefail

WORKSPACE_DIR=/home/vivado/workspace
OUT_DIR=/home/vivado/output

test_fail=0

function test_vivado {
    if docker run --rm --pid=host -v "$PWD"/ci:"$2" -w "$2" "vivado" bash --login vivado_sim.sh "$1" "$2" "$3" | grep "~~FAIL~~"
    then
        test_fail=`expr $test_fail + 1`
        echo -e "\033[01;31m[Vivado] [fail] $1"
    else
        echo -e "\033[01;32m[Vivado] [pass] $1"
    fi
}

test_vivado "register" $WORKSPACE_DIR $OUT_DIR
test_vivado "fsm" $WORKSPACE_DIR $OUT_DIR
test_vivado "vadd_const" $WORKSPACE_DIR $OUT_DIR

echo -en "\033[0m"
echo "Number of failed Vivado tests:$test_fail"
test $test_fail -eq 0