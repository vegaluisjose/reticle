use crate::backend::arch::ultrascale::lut::Lut;
use crate::backend::asm::ast as asm;
use vast::v05::ast as verilog;

pub fn lut_and_bool_bool_bool(instr: asm::Instr) -> Vec<verilog::Stmt> {
    let mut body: Vec<verilog::Stmt> = Vec::new();
    let mut lut = Lut::new_lut2("i0");
    lut.add_input(&instr.params()[0].id());
    lut.add_input(&instr.params()[1].id());
    lut.set_output(&instr.id());
    lut.set_init(8);
    body.push(verilog::Stmt::from(lut));
    body
}