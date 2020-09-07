#!/bin/bash

set -eo pipefail

CARGO_CMD=$1

$CARGO_CMD cargo build --release

$CARGO_CMD ./target/release/reticle examples/isa/scalar/register.ret -b verilog -o ci/register.v
$CARGO_CMD ./target/release/reticle examples/basic/fsm.ret -b verilog -o ci/fsm.v
$CARGO_CMD ./target/release/reticle examples/basic/vadd_const.ret -b verilog -o ci/vadd_const.v
