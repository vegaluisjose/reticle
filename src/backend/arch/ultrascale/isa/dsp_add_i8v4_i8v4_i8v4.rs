use crate::backend::arch::ultrascale::assembler::{Assembler, Emit};
use crate::backend::arch::ultrascale::prim::ast::{Dsp, DspOp};
use crate::backend::asm::ast as asm;
use crate::backend::verilog;

fn vector_input_gen(asm: &mut Assembler, dst_name: &str, src_name: &str, width: u64) {
    let wire = verilog::Decl::new_wire(dst_name, width);
    asm.add_wire(verilog::Stmt::from(wire));
    let mut concat = verilog::ExprConcat::default();
    // fix all of this later
    // dsp-length
    for i in 0..4 {
        let name = asm.fresh_vector_variable(src_name, i);
        concat.add_expr(verilog::Expr::new_ref(&name));
        // dsp-width / dsp-length - op-width
        for _ in 0..4 {
            concat.add_expr(verilog::Expr::new_ref(&asm.gnd()));
        }
    }
    let src_expr = verilog::Expr::from(concat);
    let dst_expr = verilog::Expr::new_ref(dst_name);
    let assign = verilog::Parallel::ParAssign(dst_expr, src_expr);
    asm.add_assignment(verilog::Stmt::from(assign));
}

fn vector_output_gen(asm: &mut Assembler, dst_name: &str, src_name: &str, width: u64) {
    let wire = verilog::Decl::new_wire(dst_name, width);
    asm.add_wire(verilog::Stmt::from(wire));
    // dsp-length
    for i in 0..4 {
        let name = asm.fresh_vector_variable(dst_name, i);
        let lo = verilog::Expr::new_int((i * 12) as i32);
        let hi = verilog::Expr::new_int((i * 12 + 8 - 1) as i32);
        let src_expr = verilog::Expr::new_slice(src_name, hi, lo);
        let dst_expr = verilog::Expr::new_ref(&name);
        let assign = verilog::Parallel::ParAssign(dst_expr, src_expr);
        asm.add_assignment(verilog::Stmt::from(assign));
    }
}

#[derive(Clone, Debug)]
pub struct DspAddI8V4I8V4I8V4;

impl Emit for DspAddI8V4I8V4I8V4 {
    fn emit(asm: &mut Assembler, instr: asm::Instr) {
        let instr = instr.prim().clone();
        let params: Vec<String> = instr.params().iter().map(|x| x.id()).collect();
        let mut dsp = Dsp::new_vector(DspOp::Add, instr.dst_ty().length());
        let left_wire_name = asm.new_variable_name();
        let right_wire_name = asm.new_variable_name();
        let output_wire_name = asm.new_variable_name();
        dsp.set_id(&asm.new_instance_name());
        dsp.set_clock(&asm.clock());
        dsp.set_reset(&asm.reset());
        dsp.set_left(&left_wire_name);
        dsp.set_right(&right_wire_name);
        dsp.set_output(&output_wire_name);
        vector_input_gen(asm, &left_wire_name, &params[0], dsp.width());
        vector_input_gen(asm, &right_wire_name, &params[1], dsp.width());
        vector_output_gen(asm, &instr.dst_id(), &output_wire_name, dsp.width());
        asm.add_instance(verilog::Stmt::from(dsp));
    }
}
