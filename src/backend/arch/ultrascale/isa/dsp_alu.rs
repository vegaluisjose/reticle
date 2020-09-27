use crate::backend::arch::ultrascale::assembler::{Assembler, Emit};
use crate::backend::arch::ultrascale::prim::ast::{DspVector, DspVectorConfig, DspVectorOp};
use crate::backend::asm::ast as asm;
use crate::backend::verilog;

fn emit_config(instr: &asm::Instr) -> DspVectorConfig {
    match instr.prim().op().as_ref() {
        "dsp_add_i8v4_r0_r0_r0" => {
            let len = instr.dst_ty().length();
            let mut config = DspVectorConfig::new(DspVectorOp::Add, len);
            config.set_pos("a", 0);
            config.set_pos("b", 1);
            config
        }
        "dsp_add_i8v4_r0_r0_r1" => {
            let len = instr.dst_ty().length();
            let mut config = DspVectorConfig::new(DspVectorOp::Add, len);
            config.set_pos("a", 0);
            config.set_pos("b", 1);
            config.set_pos("en_y", 2);
            config.set_reg("y", 1);
            config
        }
        "dsp_add_i8v4_r1_r1_r1" => {
            let len = instr.dst_ty().length();
            let mut config = DspVectorConfig::new(DspVectorOp::Add, len);
            config.set_pos("a", 0);
            config.set_pos("en_a", 1);
            config.set_pos("b", 2);
            config.set_pos("en_b", 3);
            config.set_pos("en_y", 4);
            config.set_reg("a", 1);
            config.set_reg("b", 1);
            config.set_reg("y", 1);
            config
        }
        "dsp_sub_i8v4_r0_r0_r0" => {
            let len = instr.dst_ty().length();
            let mut config = DspVectorConfig::new(DspVectorOp::Sub, len);
            config.set_pos("a", 0);
            config.set_pos("b", 1);
            config
        }
        "dsp_add_i8_r0_r0_r0" => {
            let mut config = DspVectorConfig::new(DspVectorOp::Add, 1);
            config.set_pos("a", 0);
            config.set_pos("b", 1);
            config
        }
        "dsp_sub_i8_r0_r0_r0" => {
            let mut config = DspVectorConfig::new(DspVectorOp::Sub, 1);
            config.set_pos("a", 0);
            config.set_pos("b", 1);
            config
        }
        "dsp_and_i8_r0_r0_r0" => {
            let mut config = DspVectorConfig::new(DspVectorOp::And, 1);
            config.set_pos("a", 0);
            config.set_pos("b", 1);
            config
        }
        "dsp_or_i8_r0_r0_r0" => {
            let mut config = DspVectorConfig::new(DspVectorOp::Or, 1);
            config.set_pos("a", 0);
            config.set_pos("b", 1);
            config
        }
        "dsp_xor_i8_r0_r0_r0" => {
            let mut config = DspVectorConfig::new(DspVectorOp::Xor, 1);
            config.set_pos("a", 0);
            config.set_pos("b", 1);
            config
        }
        "dsp_nand_i8_r0_r0_r0" => {
            let mut config = DspVectorConfig::new(DspVectorOp::Nand, 1);
            config.set_pos("a", 0);
            config.set_pos("b", 1);
            config
        }
        "dsp_nor_i8_r0_r0_r0" => {
            let mut config = DspVectorConfig::new(DspVectorOp::Nor, 1);
            config.set_pos("a", 0);
            config.set_pos("b", 1);
            config
        }
        "dsp_xnor_i8_r0_r0_r0" => {
            let mut config = DspVectorConfig::new(DspVectorOp::Xnor, 1);
            config.set_pos("a", 0);
            config.set_pos("b", 1);
            config
        }
        "dsp_add_i8_r0_r0_r1" => {
            let mut config = DspVectorConfig::new(DspVectorOp::Add, 1);
            config.set_pos("a", 0);
            config.set_pos("b", 1);
            config.set_pos("en_y", 2);
            config.set_reg("y", 1);
            config
        }
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
    config: &DspVectorConfig,
    port: &str,
) {
    let index = config.pos(port) as usize;
    let mut concat = verilog::ExprConcat::default();
    let width = instr.dst_ty().width();
    let length = config.get_param("length") as u64;
    let word = config.get_param("word") as u64;
    let pad = word - width;
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

fn emit_output(asm: &mut Assembler, instr: &asm::Instr, wire: &str, config: &DspVectorConfig) {
    let width = instr.dst_ty().width();
    let length = config.get_param("length") as u64;
    let word = config.get_param("word") as u64;
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
pub struct DspAlu;

impl Emit for DspAlu {
    fn emit(asm: &mut Assembler, instr: &asm::Instr) {
        let config = emit_config(&instr);
        let mut dsp = DspVector::new(config);
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
        emit_input(asm, &instr, &a, dsp.config(), "a");
        emit_input(asm, &instr, &b, dsp.config(), "b");
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
        if dsp.has_reg("y") {
            let en_y =
                asm.fresh_scalar_variable(&instr.indexed_param(dsp.pos("en_y") as usize).id());
            dsp.set_input("en_y", &en_y);
        }
        emit_output(asm, &instr, &y, dsp.config());
        asm.add_instance(verilog::Stmt::from(dsp));
    }
}
