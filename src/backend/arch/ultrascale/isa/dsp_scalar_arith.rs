use crate::backend::arch::ultrascale::assembler::{Assembler, Emit};
// use crate::backend::arch::ultrascale::prim::ast::{DspVectorOp, DspVector};
use crate::backend::asm::ast as asm;
// use crate::backend::verilog;

#[derive(Clone, Debug)]
pub struct DspScalarArith;

// fn emit_scalar_op(instr: &asm::Instr) -> DspVectorOp {
//     match instr.prim().op().as_ref() {
//         "dsp_add_reg_mul_i8_i8_i8_b_i8" => DspVectorOp::AddRegMul,
//         _ => unimplemented!(),
//     }
// }

impl Emit for DspScalarArith {
    fn emit(_: &mut Assembler, _: asm::Instr) {}
}
