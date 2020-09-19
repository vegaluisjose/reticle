use crate::backend::arch::ultrascale::assembler::{Assembler, Emit};
use crate::backend::arch::ultrascale::prim::ast::{DspVector, DspVectorOp};
use crate::backend::asm::ast as asm;
use crate::backend::verilog;

fn emit_op(instr: &asm::Instr) -> DspVectorOp {
    match instr.prim().op().as_ref() {
        "dsp_add_i8v4_i8v4_i8v4" => DspVectorOp::Add,
        "dsp_sub_i8v4_i8v4_i8v4" => DspVectorOp::Sub,
        "dsp_add_i8_i8_i8" => DspVectorOp::Add,
        "dsp_sub_i8_i8_i8" => DspVectorOp::Sub,
        "dsp_and_i8_i8_i8" => DspVectorOp::And,
        "dsp_or_i8_i8_i8" => DspVectorOp::Or,
        "dsp_xor_i8_i8_i8" => DspVectorOp::Xor,
        "dsp_nand_i8_i8_i8" => DspVectorOp::Nand,
        "dsp_nor_i8_i8_i8" => DspVectorOp::Nor,
        "dsp_xnor_i8_i8_i8" => DspVectorOp::Xnor,
        _ => unimplemented!(),
    }
}

fn emit_length(instr: &asm::Instr) -> u64 {
    if instr.is_vector() {
        instr.dst_ty().length()
    } else {
        1
    }
}

fn emit_wire(asm: &mut Assembler, width: u64) -> String {
    let name = asm.new_variable_name();
    let wire = verilog::Decl::new_wire(&name, width);
    asm.add_wire(verilog::Stmt::from(wire));
    name
}

fn emit_input(asm: &mut Assembler, instr: &asm::Instr, wire: &str, word: u64, index: usize) {
    let mut concat = verilog::ExprConcat::default();
    let width = instr.dst_ty().width();
    let pad = word - width;
    let length = emit_length(instr);
    for i in 0..length {
        let name = if instr.is_vector() {
            asm.fresh_vector_variable(&instr.indexed_param(index).id(), i)
        } else {
            asm.fresh_scalar_variable(&instr.indexed_param(index).id())
        };
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

fn emit_output(asm: &mut Assembler, instr: &asm::Instr, wire: &str, word: u64) {
    let width = instr.dst_ty().width();
    let length = emit_length(instr);
    for i in 0..length {
        let name = if instr.is_vector() {
            asm.fresh_vector_variable(&instr.dst_id(), i)
        } else {
            asm.fresh_scalar_variable(&instr.dst_id())
        };
        let lo = verilog::Expr::new_int((i * word) as i32);
        let hi = verilog::Expr::new_int((i * word + width - 1) as i32);
        let src_expr = verilog::Expr::new_slice(wire, hi, lo);
        let dst_expr = verilog::Expr::new_ref(&name);
        let assign = verilog::Parallel::ParAssign(dst_expr, src_expr);
        asm.add_assignment(verilog::Stmt::from(assign));
    }
}

#[derive(Clone, Debug)]
pub struct DspArith;

impl Emit for DspArith {
    fn emit(asm: &mut Assembler, instr: &asm::Instr) {
        let op = emit_op(&instr);
        let length = emit_length(instr);
        let mut dsp = DspVector::new(op, length);
        let dsp_word = dsp.get_param("word") as u64;
        let dsp_width = dsp.get_param("width") as u64;
        let a = emit_wire(asm, dsp_width);
        let b = emit_wire(asm, dsp_width);
        let y = emit_wire(asm, dsp_width);
        dsp.set_id(&asm.new_instance_name());
        dsp.set_input("vcc", &asm.vcc);
        dsp.set_input("gnd", &asm.gnd);
        dsp.set_input("clock", &asm.clock());
        dsp.set_input("reset", &asm.reset());
        dsp.set_input("a", &a);
        dsp.set_input("b", &b);
        dsp.set_output("y", &y);
        emit_input(asm, &instr, &a, dsp_word, 0);
        emit_input(asm, &instr, &b, dsp_word, 1);
        emit_output(asm, &instr, &y, dsp_word);
        asm.add_instance(verilog::Stmt::from(dsp));
    }
}