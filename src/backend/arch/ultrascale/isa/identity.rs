use crate::asm::ast as asm;
use crate::backend::arch::ultrascale::assembler::{Assembler, Emit};
use crate::backend::verilog;

#[derive(Clone, Debug)]
pub struct Identity;

impl Emit for Identity {
    fn emit(asm: &mut Assembler, instr: &asm::Instr) {
        let mut instr = instr.std().clone();
        let mut inp_expr = instr.indexed_param(0).clone();
        let mut dst_expr = instr.dst().clone();
        let inp_name = asm.fresh_scalar_variable(&inp_expr.id());
        let dst_name = asm.fresh_scalar_variable(&dst_expr.id());
        inp_expr.set_id(&inp_name);
        dst_expr.set_id(&dst_name);
        instr.set_param(inp_expr, 0);
        instr.set_dst(dst_expr);
        let stmts: Vec<verilog::Stmt> = instr.into();
        for s in stmts {
            asm.add_assignment(s);
        }
    }
}
