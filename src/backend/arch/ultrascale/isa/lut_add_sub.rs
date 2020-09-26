use crate::backend::arch::ultrascale::assembler::{Assembler, Emit};
use crate::backend::arch::ultrascale::prim::ast::{Carry, Lut};
use crate::backend::arch::ultrascale::prim::util::regs_from_init;
use crate::backend::asm::ast as asm;
use crate::backend::verilog;

fn emit_lut_init(op: &str) -> String {
    match op {
        "lut_add_i8_r0" | "lut_add_i8_r1" => "6".to_string(),
        "lut_sub_i8_r0" | "lut_sub_i8_r1" => "9".to_string(),
        _ => unimplemented!(),
    }
}

fn is_add(op: &str) -> bool {
    match op {
        "lut_add_i8_r0" => true,
        _ => false,
    }
}

fn has_reg(op: &str) -> bool {
    match op {
        "lut_add_i8_r1" => true,
        _ => false,
    }
}

#[derive(Clone, Debug)]
pub struct LutAddSub;

impl Emit for LutAddSub {
    fn emit(asm: &mut Assembler, instr: &asm::Instr) {
        let instr = instr.prim().clone();
        let params: Vec<String> = instr.params().iter().map(|x| x.id()).collect();
        let lhs = asm.fresh_scalar_variable(&params[0]);
        let rhs = asm.fresh_scalar_variable(&params[1]);
        let res = asm.fresh_scalar_variable(&instr.dst_id());
        let wire_name = asm.new_variable_name();
        let wire = verilog::Decl::new_wire(&wire_name, 8);
        let width = instr.dst_ty().width();
        let mut carry = Carry::default();
        let init = emit_lut_init(&instr.op());
        let carry_wire_name = asm.new_variable_name();
        let carry_wire = verilog::Decl::new_wire(&carry_wire_name, width);
        carry.set_id(&asm.new_instance_name());
        carry.set_input("vcc", &asm.vcc);
        carry.set_input("gnd", &asm.gnd);
        carry.set_input("a", &lhs);
        carry.set_input("b", &carry_wire_name);
        if is_add(&instr.op()) {
            carry.set_input("ci", &asm.gnd);
        } else {
            carry.set_input("ci", &asm.vcc);
        }
        asm.add_wire(verilog::Stmt::from(carry_wire));
        for i in 0..width {
            let mut lut = Lut::new_lut2();
            lut.set_attr("init", &init);
            lut.set_id(&asm.new_instance_name());
            lut.set_input("vcc", &asm.vcc);
            lut.set_input("gnd", &asm.gnd);
            if width == 1 {
                lut.set_input("a", &lhs);
                lut.set_input("b", &rhs);
                lut.set_output("y", &wire_name);
            } else {
                lut.set_input_with_index("a", &lhs, i as u32);
                lut.set_input_with_index("b", &rhs, i as u32);
                lut.set_output_with_index("y", &carry_wire_name, i as u32);
            }
            asm.add_instance(verilog::Stmt::from(lut));
        }
        if has_reg(&instr.op()) {
            let en = asm.fresh_scalar_variable(&params[2]);
            let reg_wire_name = asm.new_variable_name();
            let reg_wire = verilog::Decl::new_wire(&reg_wire_name, width);
            let regs = regs_from_init(instr.dst_ty().width(), instr.indexed_attr(0).value());
            asm.add_wire(verilog::Stmt::from(reg_wire));
            carry.set_output("y", &reg_wire_name);
            for (i, reg) in regs.iter().enumerate() {
                let mut reg = reg.clone();
                reg.set_id(&asm.new_instance_name());
                reg.set_input("vcc", &asm.vcc);
                reg.set_input("gnd", &asm.gnd);
                reg.set_input("clock", &asm.clock());
                reg.set_input("reset", &asm.reset());
                reg.set_input("en", &en);
                if width == 1 {
                    reg.set_input("a", &reg_wire_name);
                    reg.set_output("y", &res);
                } else {
                    reg.set_input_with_index("a", &reg_wire_name, i as u32);
                    reg.set_output_with_index("y", &res, i as u32);
                }
                asm.add_instance(verilog::Stmt::from(reg));
            }
        } else {
            carry.set_output("y", &res);
        }
        asm.add_instance(verilog::Stmt::from(carry));
        asm.add_wire(verilog::Stmt::from(wire));
    }
}
