use crate::backend::arch::ultrascale::assembler::{Assembler, EmitPrim};
use crate::backend::arch::ultrascale::prim::ast::Lut;
use crate::backend::asm::ast as asm;
use crate::backend::verilog;

#[derive(Clone, Debug)]
pub struct LutEqBI8I8;

impl EmitPrim for LutEqBI8I8 {
    fn emit_prim(asm: &mut Assembler, instr: asm::InstrPrim) {
        let params: Vec<String> = instr.params().iter().map(|x| x.id()).collect();
        let lhs = asm.fresh_variable(&params[0]);
        let rhs = asm.fresh_variable(&params[1]);
        let res = asm.fresh_variable(&instr.dst_id());
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
        lut_0.set_init("9009000000000000");
        lut_1.set_init("9009000000009009");
        lut_2.set_init("9009000000009009");
        lut_0.add_input_with_index(&rhs, 7);
        lut_0.add_input_with_index(&lhs, 7);
        lut_0.add_input_with_index(&rhs, 6);
        lut_0.add_input_with_index(&lhs, 6);
        lut_0.add_input(&wire_0_name);
        lut_0.add_input(&wire_1_name);
        lut_0.set_output(&res);
        lut_1.add_input_with_index(&lhs, 3);
        lut_1.add_input_with_index(&rhs, 3);
        lut_1.add_input_with_index(&rhs, 5);
        lut_1.add_input_with_index(&lhs, 5);
        lut_1.add_input_with_index(&rhs, 4);
        lut_1.add_input_with_index(&lhs, 4);
        lut_1.set_output(&wire_0_name);
        lut_2.add_input_with_index(&lhs, 0);
        lut_2.add_input_with_index(&rhs, 0);
        lut_2.add_input_with_index(&rhs, 2);
        lut_2.add_input_with_index(&lhs, 2);
        lut_2.add_input_with_index(&rhs, 1);
        lut_2.add_input_with_index(&lhs, 1);
        lut_2.set_output(&wire_1_name);
        asm.add_wire(verilog::Stmt::from(wire_0));
        asm.add_wire(verilog::Stmt::from(wire_1));
        asm.add_instance(verilog::Stmt::from(lut_0));
        asm.add_instance(verilog::Stmt::from(lut_1));
        asm.add_instance(verilog::Stmt::from(lut_2));
    }
}
