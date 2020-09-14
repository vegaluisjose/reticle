use crate::backend::arch::ultrascale::assembler::{Assembler, Emit};
use crate::backend::arch::ultrascale::prim::ast::{DspFused, DspFusedOp};
use crate::backend::asm::ast as asm;
use crate::backend::verilog;

#[derive(Clone, Debug)]
pub struct DspArith;

fn emit_op(instr: &asm::Instr) -> DspFusedOp {
    match instr.prim().op().as_ref() {
        "dsp_add_reg_mul_i8_i8_i8_b_i8" => DspFusedOp::MulAdd,
        _ => unimplemented!(),
    }
}

fn emit_wire(asm: &mut Assembler, width: u64) -> String {
    let name = asm.new_variable_name();
    let wire = verilog::Decl::new_wire(&name, width);
    asm.add_wire(verilog::Stmt::from(wire));
    name
}

fn emit_input(
    asm: &mut Assembler,
    instr: &asm::Instr,
    wire: &str,
    width: u64,
    index: usize,
    trunc: bool,
) {
    let mut concat = verilog::ExprConcat::default();
    let w = instr.dst_ty().width();
    let trunc_width = w / 2;
    let rem = if trunc {
        width - trunc_width
    } else {
        width - w
    };
    let name = asm.fresh_scalar_variable(&instr.indexed_param(index).id());
    let hi = if trunc {
        verilog::Expr::new_int((trunc_width - 1) as i32)
    } else {
        verilog::Expr::new_int((w - 1) as i32)
    };
    let lo = verilog::Expr::new_int(0);
    concat.add_expr(verilog::Expr::new_slice(&name, hi, lo));
    // extend sign
    for _ in 0..rem {
        concat.add_expr(verilog::Expr::new_index_bit(&name, (w - 1) as i32));
    }
    let src_expr = verilog::Expr::from(concat);
    let dst_expr = verilog::Expr::new_ref(wire);
    let assign = verilog::Parallel::ParAssign(dst_expr, src_expr);
    asm.add_assignment(verilog::Stmt::from(assign));
}

fn emit_output(asm: &mut Assembler, instr: &asm::Instr, wire: &str) {
    let width = instr.dst_ty().width();
    let name = asm.fresh_scalar_variable(&instr.dst_id());
    let lo = verilog::Expr::new_int(0);
    let hi = verilog::Expr::new_int((width - 1) as i32);
    let src_expr = verilog::Expr::new_slice(wire, hi, lo);
    let dst_expr = verilog::Expr::new_ref(&name);
    let assign = verilog::Parallel::ParAssign(dst_expr, src_expr);
    asm.add_assignment(verilog::Stmt::from(assign));
}

impl Emit for DspArith {
    fn emit(asm: &mut Assembler, instr: &asm::Instr) {
        let op = emit_op(&instr);
        let mut dsp = DspFused::new(op);
        let aw = dsp.get_param("aw") as u64;
        let bw = dsp.get_param("bw") as u64;
        let cw = dsp.get_param("cw") as u64;
        let yw = dsp.get_param("yw") as u64;
        let a = emit_wire(asm, aw);
        let b = emit_wire(asm, bw);
        let c = emit_wire(asm, cw);
        let y = emit_wire(asm, yw);
        let en_mul = asm.fresh_scalar_variable(&instr.indexed_param(2).id());
        dsp.set_id(&asm.new_instance_name());
        dsp.set_input("clock", &asm.clock());
        dsp.set_input("reset", &asm.reset());
        dsp.set_input("a", &a);
        dsp.set_input("b", &b);
        dsp.set_input("c", &c);
        dsp.set_input("en_mul", &en_mul);
        dsp.set_output("y", &y);
        emit_input(asm, &instr, &a, aw, 0, true);
        emit_input(asm, &instr, &b, bw, 1, true);
        emit_input(asm, &instr, &c, cw, 3, false);
        emit_output(asm, &instr, &y);
        asm.add_instance(verilog::Stmt::from(dsp));
    }
}
