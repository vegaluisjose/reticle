# Reticle (WIP, highly experimental)

## Setup, build, and install

1. Install [rustup](https://www.rust-lang.org/tools/install)
    * `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
2. Install `1.50.0` version
    ```bash
       rustup toolchain install 1.50.0
       rustup override set 1.50.0
    ```
3. Run unit tests
    * `cargo test --all`
4. Build, binaries are located in `target/release`
    * `cargo build --release`
5. Optional, install locally (binaries are normally located in `~/.cargo/bin`)
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
