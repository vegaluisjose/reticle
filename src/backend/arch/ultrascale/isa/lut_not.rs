use crate::backend::arch::ultrascale::assembler::{Assembler, Emit};
use crate::backend::arch::ultrascale::prim::ast::Lut;
use crate::backend::arch::ultrascale::prim::util::regs_from_init;
use crate::backend::asm::ast as asm;
use crate::backend::verilog;

fn has_reg(op: &str) -> bool {
    match op {
        "lut_reg_not_i8_i8" | "lut_reg_not_b_b" => true,
        "lut_not_i8_r0" | "lut_not_bool_r0" => false,
        _ => unimplemented!(),
    }
}

#[derive(Clone, Debug)]
pub struct LutNot;

impl Emit for LutNot {
    fn emit(asm: &mut Assembler, instr: &asm::Instr) {
        let instr = instr.prim().clone();
        let params: Vec<String> = instr.params().iter().map(|x| x.id()).collect();
        let inp = asm.fresh_scalar_variable(&params[0]);
        let res = asm.fresh_scalar_variable(&instr.dst_id());
        let width = instr.dst_ty().width();
        if has_reg(&instr.op()) {
            let en = asm.fresh_scalar_variable(&params[1]);
            let wire_name = asm.new_variable_name();
            let wire = verilog::Decl::new_wire(&wire_name, width);
            let regs = regs_from_init(instr.dst_ty().width(), instr.indexed_attr(0).value());
            asm.add_wire(verilog::Stmt::from(wire));
            for (i, reg) in regs.iter().enumerate() {
                let mut lut = Lut::new_lut1();
                let mut reg = reg.clone();
                lut.set_id(&asm.new_instance_name());
                lut.set_input("vcc", &asm.vcc);
                lut.set_input("gnd", &asm.gnd);
                lut.set_attr("init", "1");
                lut.set_output_with_index("y", &wire_name, i as u32);
                reg.set_id(&asm.new_instance_name());
                reg.set_input("vcc", &asm.vcc);
                reg.set_input("gnd", &asm.gnd);
                reg.set_input("clock", &asm.clock());
                reg.set_input("reset", &asm.reset());
                reg.set_input("en", &en);
                if width == 1 {
                    lut.set_input("a", &inp);
                    reg.set_input("a", &wire_name);
                    reg.set_output("y", &res);
                } else {
                    lut.set_input_with_index("a", &inp, i as u32);
                    reg.set_input_with_index("a", &wire_name, i as u32);
                    reg.set_output_with_index("y", &res, i as u32);
                }
                asm.add_instance(verilog::Stmt::from(reg));
                asm.add_instance(verilog::Stmt::from(lut));
            }
        } else {
            for i in 0..width {
                let mut lut = Lut::new_lut1();
                lut.set_id(&asm.new_instance_name());
                lut.set_attr("init", "1");
                if width == 1 {
                    lut.set_input("a", &inp);
                    lut.set_output("y", &res);
                } else {
                    lut.set_input_with_index("a", &inp, i as u32);
                    lut.set_output_with_index("y", &res, i as u32);
                }
                asm.add_instance(verilog::Stmt::from(lut));
            }
        }
    }
}
