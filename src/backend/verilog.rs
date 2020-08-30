use crate::backend::asm::ast as asm;
use vast::v05::ast as verilog;

pub use verilog::*;

impl From<asm::Prog> for verilog::Module {
    fn from(prog: asm::Prog) -> Self {
        use crate::backend::arch::ultrascale::assembler::Assembler;
        let mut assembler = Assembler::default();
        assembler.emit(prog)
    }
}
