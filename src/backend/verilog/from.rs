use crate::backend::asm::ast as asm;
use crate::backend::verilog::*;
use std::convert::From;

impl From<asm::Prog> for Module {
    fn from(prog: asm::Prog) -> Self {
        Module::new_with_name(&prog.id())
    }
}
