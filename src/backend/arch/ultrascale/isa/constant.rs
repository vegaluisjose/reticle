use crate::backend::arch::ultrascale::assembler::{Assembler, Emit};
use crate::backend::arch::ultrascale::prim::ast::Const;
use crate::backend::asm::ast as asm;
use crate::backend::verilog;

#[derive(Clone, Debug)]
pub struct Constant;

impl Emit for Constant {
    fn emit(asm: &mut Assembler, instr: asm::Instr) {
        let instr = instr.std().clone();
        assert!(!instr.is_vector()); // temporary check for only scalars
        let value = instr.indexed_attr(0).value();
        let width = instr.dst_ty().width();
        let mut constant = Const::new(width, value);
        let res = asm.fresh_variable(&instr.dst_id());
        constant.set_id(&res);
        constant.set_gnd(&asm.gnd());
        constant.set_vcc(&asm.vcc());
        asm.add_assignment(verilog::Stmt::from(constant));
    }
}
