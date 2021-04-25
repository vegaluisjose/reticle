# Reticle (WIP, highly experimental)

## Setup, build, and install

1. Install [rust](https://www.rust-lang.org/tools/install)
    * `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
2. Run unit tests
    * `cargo test --all`
3. Build, binaries are located in `target/release`
    * `cargo build --release`
4. Optional, install locally (binaries are normally located in `~/.cargo/bin`)
    * `cargo install --bin rt --bin ro --path .`

## How to use reticle compiler

1. Compile IR program to assembly (asm)
    * `rt --from ir --to asm examples/ir/add.ir`
2. Compile IR program to machine IR (xir)
    * `rt --from ir --to xir examples/ir/add.ir`
3. Compile IR program to structural Verilog (struct)
    * `rt --from ir --to struct examples/ir/add.ir`
4. Compile IR program to behavioral Verilog (behav)
    * `rt --from ir --to behav examples/ir/add.ir`
