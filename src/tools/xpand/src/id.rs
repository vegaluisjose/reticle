use crate::errors::Error;
use crate::vec_expr_try_from_expr;
use verilog::ast as vl;
use xir::ast as xir;

pub fn from_basc(instr: &xir::InstrBasc) -> Result<Vec<vl::Stmt>, Error> {
    let dst: Vec<vl::Expr> = vec_expr_try_from_expr(instr.dst())?;
    let arg: Vec<vl::Expr> = vec_expr_try_from_expr(instr.arg())?;
    let assign = vl::Parallel::Assign(dst[0].clone(), arg[0].clone());
    Ok(vec![vl::Stmt::from(assign)])
}
