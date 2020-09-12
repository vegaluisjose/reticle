use crate::backend::arch::ultrascale::assembler::{Assembler, Emit};
use crate::backend::arch::ultrascale::prim::ast::{DspOp, DspVector};
use crate::backend::asm::ast as asm;
use crate::backend::verilog;

fn emit_vector_input(asm: &mut Assembler, instr: asm::Instr, wire: &str, index: usize, pad: u64) {
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

fn emit_vector_output(asm: &mut Assembler, instr: asm::Instr, wire: &str, word: u64) {
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

fn emit_vector_wire(asm: &mut Assembler, width: u64) -> String {
    let name = asm.new_variable_name();
    let wire = verilog::Decl::new_wire(&name, width);
    asm.add_wire(verilog::Stmt::from(wire));
    name
}

fn emit_vector_op(instr: &asm::Instr) -> DspOp {
    match instr.prim().op().as_ref() {
        "dsp_add_i8v4_i8v4_i8v4" => DspOp::Add,
        _ => unimplemented!(),
    }
}

#[derive(Clone, Debug)]
pub struct DspVectorArith;

impl Emit for DspVectorArith {
    fn emit(asm: &mut Assembler, instr: asm::Instr) {
        let op = emit_vector_op(&instr);
        let mut dsp = DspVector::new(op, instr.dst_ty().length());
        let left = emit_vector_wire(asm, dsp.width());
        let right = emit_vector_wire(asm, dsp.width());
        let output = emit_vector_wire(asm, dsp.width());
        dsp.set_id(&asm.new_instance_name());
        dsp.set_input_new(&asm.clock());
        dsp.set_reset(&asm.reset());
        dsp.set_left(&left);
        dsp.set_right(&right);
        dsp.set_output(&output);
        emit_vector_input(asm, instr.clone(), &left, 0, dsp.pad());
        emit_vector_input(asm, instr.clone(), &right, 1, dsp.pad());
        emit_vector_output(asm, instr, &output, dsp.word());
        asm.add_instance(verilog::Stmt::from(dsp));
    }
}
