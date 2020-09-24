use crate::backend::arch::ultrascale::assembler::{Assembler, Emit};
use crate::backend::arch::ultrascale::prim::ast::Lut;
use crate::backend::asm::ast as asm;
use crate::backend::verilog;

fn emit_lut_init(instr: &asm::Instr) -> String {
    match instr.prim().op().as_ref() {
        "lut_and_bool_r0" | "lut_and_i8_r0" => "8".to_string(),
        "lut_or_bool_r0" | "lut_or_i8_r0" => "e".to_string(),
        "lut_xor_bool_r0" | "lut_xor_i8_r0" | "lut_neq1_bool_r0" => "6".to_string(),
        "lut_nand_bool_r0" | "lut_nand_i8_r0" => "7".to_string(),
        "lut_nor_bool_r0" | "lut_nor_i8_r0" => "1".to_string(),
        "lut_xnor_bool_r0" | "lut_xnor_i8_r0" | "lut_eq1_bool_r0" => "9".to_string(),
        _ => unimplemented!(),
    }
}

#[derive(Clone, Debug)]
pub struct LutLogical;

impl Emit for LutLogical {
    fn emit(asm: &mut Assembler, instr: &asm::Instr) {
        let params: Vec<String> = instr.params().iter().map(|x| x.id()).collect();
        let lhs = asm.fresh_scalar_variable(&params[0]);
        let rhs = asm.fresh_scalar_variable(&params[1]);
        let res = asm.fresh_scalar_variable(&instr.dst_id());
        let width = instr.dst_ty().width();
        let init = emit_lut_init(&instr);
        for i in 0..width {
            let mut lut = Lut::new_lut2();
            lut.set_attr("init", &init);
            lut.set_id(&asm.new_instance_name());
            lut.set_input("vcc", &asm.vcc);
            lut.set_input("gnd", &asm.gnd);
            if width == 1 {
                lut.set_input("a", &lhs);
                lut.set_input("b", &rhs);
                lut.set_output("y", &res);
            } else {
                lut.set_input_with_index("a", &lhs, i as u32);
                lut.set_input_with_index("b", &rhs, i as u32);
                lut.set_output_with_index("y", &res, i as u32);
            }
            asm.add_instance(verilog::Stmt::from(lut));
        }
    }
}
