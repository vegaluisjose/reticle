use crate::backend::arch::ultrascale::assembler::{Assembler, Emit};
use crate::backend::arch::ultrascale::prim::ast::Lut;
use crate::backend::asm::ast as asm;
use crate::backend::verilog;

#[derive(Clone, Debug)]
pub struct LutOr;

impl Emit for LutOr {
    fn emit(asm: &mut Assembler, instr: asm::Instr) {
        let instr = instr.prim().clone();
        let params: Vec<String> = instr.params().iter().map(|x| x.id()).collect();
        let lhs = asm.fresh_scalar_variable(&params[0]);
        let rhs = asm.fresh_scalar_variable(&params[1]);
        let res = asm.fresh_scalar_variable(&instr.dst_id());
        let width = instr.dst_ty().width();
        for i in 0..width {
            let mut lut = Lut::new_lut2();
            lut.set_init("e");
            lut.set_id(&asm.new_instance_name());
            if width == 1 {
                lut.add_input(&lhs);
                lut.add_input(&rhs);
                lut.set_output(&res);
            } else {
                lut.add_input_with_index(&lhs, i as u32);
                lut.add_input_with_index(&rhs, i as u32);
                lut.set_output_with_index(&res, i as u32);
            }
            asm.add_instance(verilog::Stmt::from(lut));
        }
    }
}
