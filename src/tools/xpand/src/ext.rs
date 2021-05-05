use crate::errors::Error;
use crate::vec_expr_try_from_expr;
use std::convert::TryFrom;
use verilog::ast as vl;
use xir::ast as xir;

// TODO: add vector type support
pub fn ext_from_basc(instr: &xir::InstrBasc) -> Result<Vec<vl::Stmt>, Error> {
    let term = instr.dst().get_term(0)?;
    if term.is_vector() {
        Err(Error::new_xpand_error("vector ext not supported yet"))
    } else {
        let index = instr.attr().get_val(0)?;
        let index = i32::try_from(index)?;
        let arg: Vec<vl::Expr> = vec_expr_try_from_expr(instr.arg())?;
        let dst: Vec<vl::Expr> = vec_expr_try_from_expr(instr.dst())?;
        let assign =
            vl::Parallel::Assign(dst[0].clone(), vl::Expr::new_index_bit(&arg[0].id(), index));
        let stmt = vl::Stmt::from(assign);
        Ok(vec![stmt])
    }
}
