use vast::v05::ast as verilog;

pub use verilog::*;

pub trait ToVerilog {
    fn to_verilog(&self) -> Module;
}
