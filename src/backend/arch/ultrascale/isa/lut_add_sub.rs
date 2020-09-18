use crate::backend::arch::ultrascale::assembler::{Assembler, Emit};
use crate::backend::arch::ultrascale::prim::ast::{Carry, Lut};
use crate::backend::asm::ast as asm;
use crate::backend::verilog;

fn emit_lut_init(instr: &asm::Instr) -> String {
    match instr.prim().op().as_ref() {
        "lut_add_i8_i8_i8" => "6".to_string(),
        "lut_sub_i8_i8_i8" => "9".to_string(),
        _ => unimplemented!(),
    }
}

fn is_add(instr: &asm::Instr) -> bool {
    match instr.prim().op().as_ref() {
        "lut_add_i8_i8_i8" => true,
        "lut_sub_i8_i8_i8" => false,
        _ => unimplemented!(),
    }
}

#[derive(Clone, Debug)]
pub struct LutAddSub;

impl Emit for LutAddSub {
    fn emit(asm: &mut Assembler, instr: &asm::Instr) {
        let params: Vec<String> = instr.params().iter().map(|x| x.id()).collect();
        let lhs = asm.fresh_scalar_variable(&params[0]);
        let rhs = asm.fresh_scalar_variable(&params[1]);
        let res = asm.fresh_scalar_variable(&instr.dst_id());
        let wire_name = asm.new_variable_name();
        let wire = verilog::Decl::new_wire(&wire_name, 8);
        let width = instr.dst_ty().width();
        let mut carry = Carry::default();
        let init = emit_lut_init(&instr);
        for i in 0..width {
            let mut lut = Lut::new_lut2();
            lut.set_attr("init", &init);
            lut.set_id(&asm.new_instance_name());
            lut.set_input("vcc", &asm.vcc);
            lut.set_input("gnd", &asm.gnd);
            lut.set_input_with_index("a", &lhs, i as u32);
            lut.set_input_with_index("b", &rhs, i as u32);
            lut.set_output_with_index("y", &wire_name, i as u32);
            asm.add_instance(verilog::Stmt::from(lut));
        }
        carry.set_id(&asm.new_instance_name());
        carry.set_input("vcc", &asm.vcc);
        carry.set_input("gnd", &asm.gnd);
        carry.set_input("a", &lhs);
        carry.set_input("b", &wire_name);
        if is_add(instr) {
            carry.set_input("ci", &asm.gnd);
        } else {
            carry.set_input("ci", &asm.vcc);
        }
        carry.set_output("y", &res);
        asm.add_instance(verilog::Stmt::from(carry));
        asm.add_wire(verilog::Stmt::from(wire));
    }
}
