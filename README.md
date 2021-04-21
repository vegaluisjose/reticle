# Reticle (WIP, highly experimental)

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

### How to run a compilation pipeline (IR-to-verilog)

1. Compile IR program to structural verilog
    * `reticle-translate examples/ir/add.ir -o add.v --fromto ir-to-struct`
2. Compile IR program to structural verilog (with placement)
    * `reticle-translate examples/ir/add.ir -o add.v --fromto ir-to-struct-placed`
3. Compile IR program to behavioral verilog
    * `reticle-translate examples/ir/add.ir -o add.v --fromto ir-to-behav`
4. Compile IR program to behavioral verilog (with dsp annotations)
    * `reticle-translate examples/ir/add.ir -o add.v --fromto ir-to-behav-dsp`
