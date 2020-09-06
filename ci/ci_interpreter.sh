#!/bin/bash

set -eo pipefail

CARGO_CMD=$1

$CARGO_CMD cargo test 
