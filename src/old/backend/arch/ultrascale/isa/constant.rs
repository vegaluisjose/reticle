use crate::asm::ast as asm;
use crate::backend::arch::ultrascale::assembler::{Assembler, Emit};
use crate::backend::arch::ultrascale::prim::ast::Const;
use crate::backend::verilog;

#[derive(Clone, Debug)]
pub struct Constant;

impl Emit for Constant {
    fn emit(asm: &mut Assembler, instr: &asm::Instr) {
        let instr = instr.std().clone();
        let width = instr.dst_ty().width();
        if instr.is_vector() {
            for i in 0..instr.dst_ty().length() {
                let value = instr.indexed_attr(i as usize).value();
                let mut constant = Const::new(width, value);
                let res = asm.fresh_vector_variable(&instr.dst_id(), i);
                constant.set_id(&res);
                constant.set_input("gnd", &asm.gnd());
                constant.set_input("vcc", &asm.vcc());
                asm.add_assignment(verilog::Stmt::from(constant));
            }
        } else {
            let value = instr.indexed_attr(0).value();
            let mut constant = Const::new(width, value);
            let res = asm.fresh_scalar_variable(&instr.dst_id());
            constant.set_id(&res);
            constant.set_input("gnd", &asm.gnd());
            constant.set_input("vcc", &asm.vcc());
            asm.add_assignment(verilog::Stmt::from(constant));
        }
    }
}
