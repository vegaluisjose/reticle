use crate::backend::asm::ast as asm;
use crate::backend::verilog::*;
use std::convert::From;

impl From<asm::Prog> for Module {
    fn from(prog: asm::Prog) -> Self {
        let mut m = Module::new_with_name(&prog.id());
        for input in prog.inputs().iter() {
            m.add_input(&input.id(), input.ty().width());
        }
        for output in prog.outputs().iter() {
            m.add_output(&output.id(), output.ty().width());
        }
        m
    }
}
