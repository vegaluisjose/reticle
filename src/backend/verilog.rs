use crate::backend::arch::ultrascale::assembler::Assembler;
use crate::backend::asm::ast as asm;
use crate::lang::ast as lang;
use vast::v05::ast as verilog;

pub use verilog::*;

impl From<asm::Prog> for verilog::Module {
    fn from(prog: asm::Prog) -> Self {
        let mut assembler = Assembler::default();
        assembler.emit(prog)
    }
}

impl From<lang::InstrStd> for verilog::Stmt {
    fn from(instr: lang::InstrStd) -> Self {
        match instr.op() {
            lang::StdOp::Identity => {
                let inp = verilog::Expr::new_ref(&instr.indexed_param(0).id());
                let out = verilog::Expr::new_ref(&instr.dst_id());
                let assign = verilog::Parallel::ParAssign(out, inp);
                verilog::Stmt::from(assign)
            }
            lang::StdOp::Const => {
                if instr.is_vector() {
                    unimplemented!()
                } else {
                    let attr = &instr.indexed_attr(0).value();
                    let width = instr.dst_ty().width();
                    let value = verilog::Expr::new_ulit_dec(width as u32, &attr.to_string());
                    let out = verilog::Expr::new_ref(&instr.dst_id());
                    let assign = verilog::Parallel::ParAssign(out, value);
                    verilog::Stmt::from(assign)
                }
            }
            _ => unimplemented!(),
        }
    }
}
