# Reticle

## Setup, build, and install

1. Install [rust](https://www.rust-lang.org/tools/install)
    * `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
2. Install Z3 solver (for placement only)
    * `pip3 install --user z3-solver`
3. Build
    * `cargo build --release`
4. Install locally (normally in `~/.cargo/bin`)
    * `cargo install --bin reticle-translate --bin reticle-optimize --bin reticle-place --path .`

## Verify installation

### How to compile in phases

1. Compile IR program to assembly
    * `reticle-translate examples/ir/add.ir -o add.asm --fromto ir-to-asm`
2. Place assembly program
    * `reticle-place add.asm -o add_placed.asm`
3. Compile assembly to xl
    * `reticle-translate add_placed.asm -o add.xl --fromto asm-to-xl`
4. Compile xl to structural verilog
    * `reticle-translate add.xl -o add.v --fromto xl-to-struct`

### How to run all compilation pipeline at once

1. Compile IR program to structural verilog
    * `reticle-translate examples/ir/add.ir -o add.v --fromto ir-to-struct`

## How to install Xilinx Vivado on Docker

1. Create an account in [Xilinx](https://www.xilinx.com/registration/create-account.html)
2. Download Vivado web installer e.g., for `2020.1` it would be `Xilinx_Unified_2020.1_0602_1208_Lin64.bin`. The file name is set in both `credential.sh` and `install.sh`
3. Move `Xilinx_Unified_2020.1_0602_1208_Lin64.bin` to docker folder in this repository
4. Make sure docker image size limit is greater than 90GB, the final image is around 35GB
5. Change to docker directory `cd docker`
6. Run `./build.sh` script for setting up all Docker images
