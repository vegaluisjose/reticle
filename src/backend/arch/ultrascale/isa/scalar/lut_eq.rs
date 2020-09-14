use crate::backend::arch::ultrascale::assembler::{Assembler, Emit};
use crate::backend::arch::ultrascale::prim::ast::{Lut, Reg};
use crate::backend::asm::ast as asm;
use crate::backend::verilog;

fn has_reg(op: &str) -> bool {
    match op {
        "lut_eq_b_i8_i8" => false,
        "lut_reg_eq_b_i8_i8" => true,
        _ => unimplemented!(),
    }
}

fn is_eq(op: &str) -> bool {
    match op {
        "lut_eq_b_i8_i8" => true,
        "lut_reg_eq_b_i8_i8" => true,
        "lut_neq_b_i8_i8" => false,
        "lut_reg_neq_b_i8_i8" => false,
        _ => unimplemented!(),
    }
}

#[derive(Clone, Debug)]
pub struct LutEq;

impl Emit for LutEq {
    fn emit(asm: &mut Assembler, instr: &asm::Instr) {
        let instr = instr.prim().clone();
        let params: Vec<String> = instr.params().iter().map(|x| x.id()).collect();
        let lhs = asm.fresh_scalar_variable(&params[0]);
        let rhs = asm.fresh_scalar_variable(&params[1]);
        let res = asm.fresh_scalar_variable(&instr.dst_id());
        let wire_0_name = asm.new_variable_name();
        let wire_1_name = asm.new_variable_name();
        let wire_0 = verilog::Decl::new_wire(&wire_0_name, 1);
        let wire_1 = verilog::Decl::new_wire(&wire_1_name, 1);
        let mut lut_0 = Lut::new_lut6();
        let mut lut_1 = Lut::new_lut6();
        let mut lut_2 = Lut::new_lut6();
        lut_0.set_id(&asm.new_instance_name());
        lut_1.set_id(&asm.new_instance_name());
        lut_2.set_id(&asm.new_instance_name());
        if is_eq(&instr.op) {
            lut_0.set_attr("init", "9009000000000000");
            lut_1.set_attr("init", "9009000000009009");
            lut_2.set_attr("init", "9009000000009009");
        } else {
            lut_0.set_attr("init", "FFFFFFFFFFFF6FF6");
            lut_1.set_attr("init", "6FF6FFFFFFFF6FF6");
            lut_2.set_attr("init", "6FF6FFFFFFFF6FF6");
        }
        lut_0.set_input_with_index("a", &rhs, 7);
        lut_0.set_input_with_index("b", &lhs, 7);
        lut_0.set_input_with_index("c", &rhs, 6);
        lut_0.set_input_with_index("d", &lhs, 6);
        lut_0.set_input("e", &wire_0_name);
        lut_0.set_input("f", &wire_1_name);
        lut_1.set_input_with_index("a", &lhs, 3);
        lut_1.set_input_with_index("b", &rhs, 3);
        lut_1.set_input_with_index("c", &rhs, 5);
        lut_1.set_input_with_index("d", &lhs, 5);
        lut_1.set_input_with_index("e", &rhs, 4);
        lut_1.set_input_with_index("f", &lhs, 4);
        lut_1.set_output("y", &wire_0_name);
        lut_2.set_input_with_index("a", &lhs, 0);
        lut_2.set_input_with_index("b", &rhs, 0);
        lut_2.set_input_with_index("c", &rhs, 2);
        lut_2.set_input_with_index("d", &lhs, 2);
        lut_2.set_input_with_index("e", &rhs, 1);
        lut_2.set_input_with_index("f", &lhs, 1);
        lut_2.set_output("y", &wire_1_name);
        let has_reg = has_reg(&instr.op());
        if has_reg {
            let wire_r_name = asm.new_variable_name();
            let wire_r = verilog::Decl::new_wire(&wire_r_name, 1);
            asm.add_wire(verilog::Stmt::from(wire_r));
            lut_0.set_output("y", &wire_r_name);
            let mut reg = if instr.indexed_attr(0).value() == 0 {
                Reg::new_fdre()
            } else {
                Reg::new_fdse()
            };
            let en = asm.fresh_scalar_variable(&params[3]);
            reg.set_id(&asm.new_instance_name());
            reg.set_input("clock", &asm.clock());
            reg.set_input("reset", &asm.reset());
            reg.set_input("en", &en);
            reg.set_input("a", &wire_r_name);
            reg.set_output("y", &res);
        } else {
            lut_0.set_output("y", &res);
        }
        asm.add_wire(verilog::Stmt::from(wire_0));
        asm.add_wire(verilog::Stmt::from(wire_1));
        asm.add_instance(verilog::Stmt::from(lut_0));
        asm.add_instance(verilog::Stmt::from(lut_1));
        asm.add_instance(verilog::Stmt::from(lut_2));
    }
}
