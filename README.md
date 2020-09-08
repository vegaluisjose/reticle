# Reticle

## Build environment

1. Create an account in [Xilinx](https://www.xilinx.com/registration/create-account.html)
2. Download Vivado web installer e.g., for `2020.1` it would be `Xilinx_Unified_2020.1_0602_1208_Lin64.bin`. The file name is set in both `credential.sh` and `install.sh`
3. Move `Xilinx_Unified_2020.1_0602_1208_Lin64.bin` to docker folder in this repository
4. Make sure docker image size limit is greater than 90GB, the final image is around 35GB
5. Change to docker directory `cd docker`
6. Run `./build.sh` script for setting up all Docker images

## Build compiler (still WIP)

If wants to try the compiler without docker, then the compiler just requires Rust. Do the following:

1. Install [Rust](https://www.rust-lang.org/tools/install)
2. Run `cargo build --release`
3. Compile to different backends
```bash
# Compiling to reticle (just figure out locations)
./target/release/reticle examples/basic/fsm.ret -b loc
# Compiling to reticle asm
./target/release/reticle examples/basic/fsm.ret -b asm
# Compiling to reticle verilog
./target/release/reticle examples/basic/fsm.ret -b verilog
# Check reticle options
./target/release/reticle -h
```

## Run interpreter

Run `cargo test` to execute the regression on all `examples` programs

## Run continuous integration

1. After building all Docker images, run `make` or `bash ci/run.sh` to run regression
