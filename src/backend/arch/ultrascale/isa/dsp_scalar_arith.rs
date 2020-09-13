use crate::backend::arch::ultrascale::assembler::{Assembler, Emit};
use crate::backend::arch::ultrascale::prim::ast::{DspScalarOp, DspScalar};
use crate::backend::asm::ast as asm;
use crate::backend::verilog;

#[derive(Clone, Debug)]
pub struct DspScalarArith;

fn emit_scalar_op(instr: &asm::Instr) -> DspScalarOp {
    match instr.prim().op().as_ref() {
        "dsp_add_reg_mul_i8_i8_i8_b_i8" => DspScalarOp::MulAdd,
        _ => unimplemented!(),
    }
}

fn emit_wire(asm: &mut Assembler, width: u64) -> String {
    let name = asm.new_variable_name();
    let wire = verilog::Decl::new_wire(&name, width);
    asm.add_wire(verilog::Stmt::from(wire));
    name
}

impl Emit for DspScalarArith {
    fn emit(asm: &mut Assembler, instr: asm::Instr) {
        let op = emit_scalar_op(&instr);
        let mut dsp = DspScalar::new(op);
        let a = emit_wire(asm, dsp.get_width("a"));
        let b = emit_wire(asm, dsp.get_width("b"));
        let c = emit_wire(asm, dsp.get_width("c"));
        let y = emit_wire(asm, dsp.get_width("y"));
        let en_mul = asm.fresh_scalar_variable(&instr.indexed_param(2).id());
        dsp.set_id(&asm.new_instance_name());
        dsp.set_input("clock", &asm.clock());
        dsp.set_input("reset", &asm.reset());
        dsp.set_input("a", &a);
        dsp.set_input("b", &b);
        dsp.set_input("c", &c);
        dsp.set_input("en_mul", &en_mul);
        dsp.set_output("y", &y);
        asm.add_instance(verilog::Stmt::from(dsp));
    }
}
