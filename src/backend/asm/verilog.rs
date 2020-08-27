use crate::backend::arch::ultrascale::assembler::Assembler;
use crate::backend::asm::ast as asm;
use vast::v05::ast as verilog;

pub use verilog::*;

pub trait ToVerilog {
    fn to_verilog(&self) -> Module;
}

impl From<asm::Prog> for Module {
    fn from(prog: asm::Prog) -> Self {
        let assembler = Assembler::new(prog);
        assembler.to_verilog()
    }
}
