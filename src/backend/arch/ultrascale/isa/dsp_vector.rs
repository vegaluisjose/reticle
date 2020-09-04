use crate::backend::arch::ultrascale::assembler::{Assembler, Emit};
use crate::backend::arch::ultrascale::prim::ast::{Dsp, DspOp};
use crate::backend::asm::ast as asm;
use crate::backend::verilog;

fn vector_input_gen(asm: &mut Assembler, instr: asm::Instr, wire: &str, index: usize, pad: u64) {
    let mut concat = verilog::ExprConcat::default();
    let length = instr.dst_ty().length();
    for i in 0..length {
        let name = asm.fresh_vector_variable(&instr.indexed_param(index).id(), i);
        concat.add_expr(verilog::Expr::new_ref(&name));
        for _ in 0..pad {
            concat.add_expr(verilog::Expr::new_ref(&asm.gnd()));
        }
    }
    let src_expr = verilog::Expr::from(concat);
    let dst_expr = verilog::Expr::new_ref(wire);
    let assign = verilog::Parallel::ParAssign(dst_expr, src_expr);
    asm.add_assignment(verilog::Stmt::from(assign));
}

fn vector_output_gen(asm: &mut Assembler, instr: asm::Instr, wire: &str, word: u64) {
    let length = instr.dst_ty().length();
    let width = instr.dst_ty().width();
    for i in 0..length {
        let name = asm.fresh_vector_variable(&instr.dst_id(), i);
        let lo = verilog::Expr::new_int((i * word) as i32);
        let hi = verilog::Expr::new_int((i * word + width - 1) as i32);
        let src_expr = verilog::Expr::new_slice(wire, hi, lo);
        let dst_expr = verilog::Expr::new_ref(&name);
        let assign = verilog::Parallel::ParAssign(dst_expr, src_expr);
        asm.add_assignment(verilog::Stmt::from(assign));
    }
}

fn vector_wire_gen(asm: &mut Assembler, width: u64) -> String {
    let name = asm.new_variable_name();
    let wire = verilog::Decl::new_wire(&name, width);
    asm.add_wire(verilog::Stmt::from(wire));
    name
}

#[derive(Clone, Debug)]
pub struct DspVector;

impl Emit for DspVector {
    fn emit(asm: &mut Assembler, instr: asm::Instr) {
        let mut dsp = Dsp::new_vector(DspOp::Add, instr.dst_ty().length());
        let left = vector_wire_gen(asm, dsp.width());
        let right = vector_wire_gen(asm, dsp.width());
        let output = vector_wire_gen(asm, dsp.width());
        dsp.set_id(&asm.new_instance_name());
        dsp.set_clock(&asm.clock());
        dsp.set_reset(&asm.reset());
        dsp.set_left(&left);
        dsp.set_right(&right);
        dsp.set_output(&output);
        vector_input_gen(asm, instr.clone(), &left, 0, dsp.pad());
        vector_input_gen(asm, instr.clone(), &right, 1, dsp.pad());
        vector_output_gen(asm, instr, &output, dsp.word());
        asm.add_instance(verilog::Stmt::from(dsp));
    }
}
