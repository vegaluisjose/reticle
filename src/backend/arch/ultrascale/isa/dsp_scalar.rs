use crate::backend::arch::ultrascale::assembler::{Assembler, Emit};
use crate::backend::arch::ultrascale::prim::ast::{Dsp, DspOp};
use crate::backend::asm::ast as asm;
use crate::backend::verilog;

#[derive(Clone, Debug)]
pub struct DspScalar;

fn emit_scalar_op(instr: &asm::Instr) -> DspOp {
    match instr.prim().op().as_ref() {
        "dsp_add_reg_mul_i8_i8_i8_b_i8" => DspOp::AddRegMul,
        _ => unimplemented!(),
    }
}

impl Emit for DspScalar {
    fn emit(asm: &mut Assembler, instr: asm::Instr) {
        let op = emit_scalar_op(&instr);
        let params: Vec<String> = instr.params().iter().map(|x| x.id()).collect();
        let mut dsp = Dsp::new_scalar(op.clone());
        dsp.set_id(&asm.new_instance_name());
        dsp.set_clock(&asm.clock());
        dsp.set_reset(&asm.reset());
        if op == DspOp::AddRegMul {
            let en = asm.fresh_scalar_variable(&params[2]);
            dsp.set_en(&en);
        }
        asm.add_instance(verilog::Stmt::from(dsp));
    }
}
