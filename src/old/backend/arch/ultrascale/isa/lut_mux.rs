use crate::asm::ast as asm;
use crate::backend::arch::ultrascale::assembler::{Assembler, Emit};
use crate::backend::arch::ultrascale::prim::ast::Lut;
use crate::backend::arch::ultrascale::prim::util::regs_from_init;
use crate::backend::verilog;

fn has_reg(op: &str) -> bool {
    match op {
        "lut_mux_i8_r1" => true,
        _ => false,
    }
}

#[derive(Clone, Debug)]
pub struct LutMux;

impl Emit for LutMux {
    fn emit(asm: &mut Assembler, instr: &asm::Instr) {
        let instr = instr.phy().clone();
        let params: Vec<String> = instr.params().iter().map(|x| x.id()).collect();
        let con = asm.fresh_scalar_variable(&params[0]);
        let tru = asm.fresh_scalar_variable(&params[1]);
        let fal = asm.fresh_scalar_variable(&params[2]);
        let res = asm.fresh_scalar_variable(&instr.dst_id());
        let width = instr.dst_ty().width();
        if has_reg(&instr.op()) {
            let en = asm.fresh_scalar_variable(&params[3]);
            let wire_name = asm.new_variable_name();
            let wire = verilog::Decl::new_wire(&wire_name, width);
            let regs = regs_from_init(instr.dst_ty().width(), instr.indexed_attr(0).value());
            asm.add_wire(verilog::Stmt::from(wire));
            for (i, reg) in regs.iter().enumerate() {
                let mut lut = Lut::new_lut3();
                let mut reg = reg.clone();
                lut.set_id(&asm.new_instance_name());
                lut.set_input("vcc", &asm.vcc);
                lut.set_input("gnd", &asm.gnd);
                lut.set_attr("init", "ac");
                lut.set_input("c", &con);
                reg.set_id(&asm.new_instance_name());
                reg.set_input("vcc", &asm.vcc);
                reg.set_input("gnd", &asm.gnd);
                reg.set_input("clock", &asm.clock());
                reg.set_input("reset", &asm.reset());
                reg.set_input("en", &en);
                if width == 1 {
                    lut.set_input("a", &tru);
                    lut.set_input("b", &fal);
                    lut.set_output("y", &wire_name);
                    reg.set_input("a", &wire_name);
                    reg.set_output("y", &res);
                } else {
                    lut.set_input_with_index("a", &tru, i as u32);
                    lut.set_input_with_index("b", &fal, i as u32);
                    lut.set_output_with_index("y", &wire_name, i as u32);
                    reg.set_input_with_index("a", &wire_name, i as u32);
                    reg.set_output_with_index("y", &res, i as u32);
                }
                asm.add_instance(verilog::Stmt::from(reg));
                asm.add_instance(verilog::Stmt::from(lut));
            }
        } else {
            for i in 0..width {
                let mut lut = Lut::new_lut3();
                lut.set_id(&asm.new_instance_name());
                lut.set_attr("init", "ac");
                lut.set_input("c", &con);
                if width == 1 {
                    lut.set_input("a", &tru);
                    lut.set_input("b", &fal);
                    lut.set_output("y", &res);
                } else {
                    lut.set_input_with_index("a", &tru, i as u32);
                    lut.set_input_with_index("b", &fal, i as u32);
                    lut.set_output_with_index("y", &res, i as u32);
                }
                asm.add_instance(verilog::Stmt::from(lut));
            }
        }
    }
}