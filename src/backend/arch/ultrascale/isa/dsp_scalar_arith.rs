use crate::backend::arch::ultrascale::assembler::{Assembler, Emit};
use crate::backend::arch::ultrascale::prim::ast::{DspScalarOp, DspScalar};
use crate::backend::asm::ast as asm;
use crate::backend::verilog;

#[derive(Clone, Debug)]
pub struct DspScalarArith;

fn emit_scalar_op(instr: &asm::Instr) -> DspScalarOp {
    match instr.prim().op().as_ref() {
        "dsp_add_reg_mul_i8_i8_i8_b_i8" => DspScalarOp::MulAdd,
        _ => unimplemented!(),
    }
}

impl Emit for DspScalarArith {
    fn emit(asm: &mut Assembler, instr: asm::Instr) {
        let op = emit_scalar_op(&instr);
        let mut dsp = DspScalar::new(op);
        dsp.set_id(&asm.new_instance_name());
        dsp.set_input("clock", &asm.clock());
        dsp.set_input("reset", &asm.reset());
        asm.add_instance(verilog::Stmt::from(dsp));
    }
}
