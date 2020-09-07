#!/bin/bash

set -eo pipefail

DOCKER_RUST=$1

$DOCKER_RUST cargo build --release

$DOCKER_RUST ./target/release/reticle examples/isa/scalar/register.ret -b verilog -o ci/register.v
$DOCKER_RUST ./target/release/reticle examples/basic/fsm.ret -b verilog -o ci/fsm.v
$DOCKER_RUST ./target/release/reticle examples/basic/vadd_const.ret -b verilog -o ci/vadd_const.v
