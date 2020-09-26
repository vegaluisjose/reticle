use crate::backend::arch::ultrascale::assembler::{Assembler, Emit};
use crate::backend::arch::ultrascale::prim::ast::{DspFused, DspFusedConfig, DspFusedOp};
use crate::backend::asm::ast as asm;
use crate::backend::verilog;

#[derive(Clone, Debug)]
pub struct DspMulArith;

fn emit_config(instr: &asm::Instr) -> DspFusedConfig {
    match instr.prim().op().as_ref() {
        "dsp_mul_i8_r0_r0_r0" => {
            let mut config = DspFusedConfig::new(DspFusedOp::Mul);
            config.set_pos("a", 0);
            config.set_pos("b", 1);
            config
        }
        "dsp_muladd_i8_r0_r0_r0_r0_r0_a_b_c" => {
            let mut config = DspFusedConfig::new(DspFusedOp::MulAdd);
            config.set_pos("a", 0);
            config.set_pos("b", 1);
            config.set_pos("c", 2);
            config
        }
        "dsp_muladd_i8_r0_r0_r0_r1_r0_a_b_c" => {
            let mut config = DspFusedConfig::new(DspFusedOp::MulAdd);
            config.set_reg("mul", 1);
            config.set_pos("a", 0);
            config.set_pos("b", 1);
            config.set_pos("en_mul", 2);
            config.set_pos("c", 3);
            config
        }
        "dsp_muladd_i8_r1_r1_r0_r1_r1_a_b_c" => {
            let mut config = DspFusedConfig::new(DspFusedOp::MulAdd);
            config.set_reg("a", 1);
            config.set_reg("b", 1);
            config.set_reg("mul", 1);
            config.set_reg("y", 1);
            config.set_pos("a", 0);
            config.set_pos("en_a", 1);
            config.set_pos("b", 2);
            config.set_pos("en_b", 3);
            config.set_pos("en_mul", 4);
            config.set_pos("c", 5);
            config.set_pos("en_y", 6);
            config
        }
        "dsp_muladd_i8_r1_r1_r0_r1_r1_c_a_b" => {
            let mut config = DspFusedConfig::new(DspFusedOp::MulAdd);
            config.set_reg("a", 1);
            config.set_reg("b", 1);
            config.set_reg("mul", 1);
            config.set_reg("y", 1);
            config.set_pos("c", 0);
            config.set_pos("a", 1);
            config.set_pos("en_a", 2);
            config.set_pos("b", 3);
            config.set_pos("en_b", 4);
            config.set_pos("en_mul", 5);
            config.set_pos("en_y", 6);
            config
        }
        _ => unimplemented!(),
    }
}

fn emit_wire(asm: &mut Assembler, width: i64) -> String {
    let name = asm.new_variable_name();
    let wire = verilog::Decl::new_wire(&name, width as u64);
    asm.add_wire(verilog::Stmt::from(wire));
    name
}

fn emit_input(
    asm: &mut Assembler,
    instr: &asm::Instr,
    wire: &str,
    width: i64,
    index: i64,
) {
    let mut concat = verilog::ExprConcat::default();
    let ty_width = instr.dst_ty().width();
    let rem = width - ty_width as i64;
    let name = asm.fresh_scalar_variable(&instr.indexed_param(index as usize).id());
    let hi = verilog::Expr::new_int((ty_width - 1) as i32);
    let lo = verilog::Expr::new_int(0);
    concat.add_expr(verilog::Expr::new_slice(&name, hi, lo));
    // extend sign
    for _ in 0..rem {
        concat.add_expr(verilog::Expr::new_index_bit(&name, (ty_width - 1) as i32));
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

impl Emit for DspMulArith {
    fn emit(asm: &mut Assembler, instr: &asm::Instr) {
        let config = emit_config(&instr);
        let mut dsp = DspFused::new(config);
        let a = emit_wire(asm, dsp.width("a"));
        let b = emit_wire(asm, dsp.width("b"));
        let y = emit_wire(asm, dsp.width("y"));
        dsp.set_id(&asm.new_instance_name());
        dsp.set_input("vcc", &asm.vcc);
        dsp.set_input("gnd", &asm.gnd);
        dsp.set_input("clock", &asm.clock());
        dsp.set_input("reset", &asm.reset());
        dsp.set_input("a", &a);
        dsp.set_input("b", &b);
        dsp.set_output("y", &y);
        emit_input(asm, &instr, &a, dsp.width("a"), dsp.pos("a"));
        emit_input(asm, &instr, &b, dsp.width("b"), dsp.pos("b"));
        if !dsp.op().is_mul() {
            let c = emit_wire(asm, dsp.width("c"));
            dsp.set_input("c", &c);
            emit_input(asm, &instr, &c, dsp.width("c"), dsp.pos("c"));
        }
        if dsp.has_reg("a") {
            let en_a =
                asm.fresh_scalar_variable(&instr.indexed_param(dsp.pos("en_a") as usize).id());
            dsp.set_input("en_a", &en_a);
        }
        if dsp.has_reg("b") {
            let en_b =
                asm.fresh_scalar_variable(&instr.indexed_param(dsp.pos("en_b") as usize).id());
            dsp.set_input("en_b", &en_b);
        }
        if dsp.has_reg("c") {
            let en_c =
                asm.fresh_scalar_variable(&instr.indexed_param(dsp.pos("en_c") as usize).id());
            dsp.set_input("en_c", &en_c);
        }
        if dsp.has_reg("mul") {
            let en_mul =
                asm.fresh_scalar_variable(&instr.indexed_param(dsp.pos("en_mul") as usize).id());
            dsp.set_input("en_mul", &en_mul);
        }
        if dsp.has_reg("y") {
            let en_y =
                asm.fresh_scalar_variable(&instr.indexed_param(dsp.pos("en_y") as usize).id());
            dsp.set_input("en_y", &en_y);
        }
        emit_output(asm, &instr, &y);
        asm.add_instance(verilog::Stmt::from(dsp));
    }
}
