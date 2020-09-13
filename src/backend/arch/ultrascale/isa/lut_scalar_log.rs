use crate::backend::arch::ultrascale::assembler::{Assembler, Emit};
use crate::backend::arch::ultrascale::prim::ast::Lut;
use crate::backend::asm::ast as asm;
use crate::backend::verilog;

fn emit_lut_init(instr: &asm::Instr) -> String {
    match instr.prim().op().as_ref() {
        "lut_and_b_b_b" => "8".to_string(),
        "lut_or_b_b_b" => "e".to_string(),
        _ => unimplemented!(),
    }
}

#[derive(Clone, Debug)]
pub struct LutScalarLog;

impl Emit for LutScalarLog {
    fn emit(asm: &mut Assembler, instr: asm::Instr) {
        let params: Vec<String> = instr.params().iter().map(|x| x.id()).collect();
        let lhs = asm.fresh_scalar_variable(&params[0]);
        let rhs = asm.fresh_scalar_variable(&params[1]);
        let res = asm.fresh_scalar_variable(&instr.dst_id());
        let width = instr.dst_ty().width();
        let init = emit_lut_init(&instr);
        for i in 0..width {
            let mut lut = Lut::new_lut2();
            lut.set_init(&init);
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
