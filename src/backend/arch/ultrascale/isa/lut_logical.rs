use crate::asm::ast as asm;
use crate::backend::arch::ultrascale::assembler::{Assembler, Emit};
use crate::backend::arch::ultrascale::prim::ast::Lut;
use crate::backend::arch::ultrascale::prim::util::regs_from_init;
use crate::backend::verilog;

fn emit_lut_init(op: &str) -> String {
    match op {
        "lut_and_bool_r0" | "lut_and_i8_r0" | "lut_and_bool_r1" | "lut_and_i8_r1" => {
            "8".to_string()
        }
        "lut_or_bool_r0" | "lut_or_i8_r0" | "lut_or_bool_r1" | "lut_or_i8_r1" => "e".to_string(),
        "lut_xor_bool_r0" | "lut_xor_i8_r0" | "lut_neq1_bool_r0" | "lut_xor_bool_r1"
        | "lut_xor_i8_r1" | "lut_neq1_bool_r1" => "6".to_string(),
        "lut_nand_bool_r0" | "lut_nand_i8_r0" | "lut_nand_bool_r1" | "lut_nand_i8_r1" => {
            "7".to_string()
        }
        "lut_nor_bool_r0" | "lut_nor_i8_r0" | "lut_nor_bool_r1" | "lut_nor_i8_r1" => {
            "1".to_string()
        }
        "lut_xnor_bool_r0" | "lut_xnor_i8_r0" | "lut_eq1_bool_r0" | "lut_xnor_bool_r1"
        | "lut_xnor_i8_r1" | "lut_eq1_bool_r1" => "9".to_string(),
        "lut_not_i8_r0" | "lut_not_i8_r1" | "lut_not_bool_r0" | "lut_not_bool_r1" => {
            "1".to_string()
        }
        _ => unimplemented!(),
    }
}

fn has_reg(op: &str) -> bool {
    match op {
        "lut_and_bool_r1" | "lut_and_i8_r1" | "lut_or_bool_r1" | "lut_or_i8_r1"
        | "lut_xor_bool_r1" | "lut_xor_i8_r1" | "lut_neq1_bool_r1" | "lut_nand_bool_r1"
        | "lut_nand_i8_r1" | "lut_nor_bool_r1" | "lut_nor_i8_r1" | "lut_xnor_bool_r1"
        | "lut_xnor_i8_r1" | "lut_eq1_bool_r1" | "lut_not_i8_r1" | "lut_not_bool_r1" => true,
        _ => false,
    }
}

fn is_not(op: &str) -> bool {
    match op {
        "lut_not_i8_r0" | "lut_not_i8_r1" | "lut_not_bool_r0" | "lut_not_bool_r1" => true,
        _ => false,
    }
}

#[derive(Clone, Debug)]
pub struct LutLogical;

impl Emit for LutLogical {
    fn emit(asm: &mut Assembler, instr: &asm::Instr) {
        let instr = instr.prim().clone();
        let params: Vec<String> = instr.params().iter().map(|x| x.id()).collect();
        let lhs = asm.fresh_scalar_variable(&params[0]);
        let has_reg = has_reg(&instr.op());
        let is_not = is_not(&instr.op());
        let res = if has_reg {
            asm.new_variable_name()
        } else {
            asm.fresh_scalar_variable(&instr.dst_id())
        };
        let width = instr.dst_ty().width();
        let init = emit_lut_init(&instr.op());
        for i in 0..width {
            let mut lut = if is_not {
                Lut::new_lut1()
            } else {
                Lut::new_lut2()
            };
            lut.set_attr("init", &init);
            lut.set_id(&asm.new_instance_name());
            lut.set_input("vcc", &asm.vcc);
            lut.set_input("gnd", &asm.gnd);
            if width == 1 {
                lut.set_input("a", &lhs);
                lut.set_output("y", &res);
            } else {
                lut.set_input_with_index("a", &lhs, i as u32);
                lut.set_output_with_index("y", &res, i as u32);
            }
            if !is_not {
                let rhs = asm.fresh_scalar_variable(&params[1]);
                if width == 1 {
                    lut.set_input("b", &rhs);
                } else {
                    lut.set_input_with_index("b", &rhs, i as u32);
                }
            }
            asm.add_instance(verilog::Stmt::from(lut));
        }
        if has_reg {
            let en = if is_not {
                asm.fresh_scalar_variable(&params[1])
            } else {
                asm.fresh_scalar_variable(&params[2])
            };
            let out = asm.fresh_scalar_variable(&instr.dst_id());
            let wire = verilog::Decl::new_wire(&res, width);
            let regs = regs_from_init(instr.dst_ty().width(), instr.indexed_attr(0).value());
            asm.add_wire(verilog::Stmt::from(wire));
            for (i, reg) in regs.iter().enumerate() {
                let mut reg = reg.clone();
                reg.set_id(&asm.new_instance_name());
                reg.set_input("vcc", &asm.vcc);
                reg.set_input("gnd", &asm.gnd);
                reg.set_input("clock", &asm.clock());
                reg.set_input("reset", &asm.reset());
                reg.set_input("en", &en);
                if width == 1 {
                    reg.set_input("a", &res);
                    reg.set_output("y", &out);
                } else {
                    reg.set_input_with_index("a", &res, i as u32);
                    reg.set_output_with_index("y", &out, i as u32);
                }
                asm.add_instance(verilog::Stmt::from(reg));
            }
        }
    }
}
