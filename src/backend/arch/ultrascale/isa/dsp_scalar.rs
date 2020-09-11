use crate::backend::arch::ultrascale::assembler::{Assembler, Emit};
use crate::backend::arch::ultrascale::prim::ast::{Dsp, DspOp};
use crate::backend::asm::ast as asm;
use crate::backend::verilog;

#[derive(Clone, Debug)]
pub struct DspScalar;

impl Emit for DspScalar {
    fn emit(asm: &mut Assembler, _: asm::Instr) {
        let mut dsp = Dsp::new_scalar(DspOp::Add);
        dsp.set_id(&asm.new_instance_name());
        dsp.set_clock(&asm.clock());
        dsp.set_reset(&asm.reset());
        asm.add_instance(verilog::Stmt::from(dsp));
    }
}
