use crate::errors::Error;
use crate::vec_expr_try_from_expr;
use std::convert::TryFrom;
use verilog::ast as vl;
use xir::ast as xir;

// TODO: add vector type support
pub fn from_basc(instr: &xir::InstrBasc) -> Result<Vec<vl::Stmt>, Error> {
    match instr.attr().tup() {
        Some(tup) if tup.term().len() == 1 => {
            let index = instr.attr().get_val(0)?;
            let index = i32::try_from(index)?;
            let arg: Vec<vl::Expr> = vec_expr_try_from_expr(instr.arg())?;
            let dst: Vec<vl::Expr> = vec_expr_try_from_expr(instr.dst())?;
            let assign =
                vl::Parallel::Assign(dst[0].clone(), vl::Expr::new_index_bit(&arg[0].id(), index));
            let stmt = vl::Stmt::from(assign);
            Ok(vec![stmt])
        }
        Some(tup) if tup.term().len() == 2 => {
            let low = instr.attr().get_val(0)?;
            let low = i32::try_from(low)?;
            let low = vl::Expr::new_int(low);
            let high = instr.attr().get_val(1)?;
            let high = i32::try_from(high)?;
            let high = vl::Expr::new_int(high);
            let arg: Vec<vl::Expr> = vec_expr_try_from_expr(instr.arg())?;
            let dst: Vec<vl::Expr> = vec_expr_try_from_expr(instr.dst())?;
            let assign =
                vl::Parallel::Assign(dst[0].clone(), vl::Expr::new_slice(&arg[0].id(), high, low));
            let stmt = vl::Stmt::from(assign);
            Ok(vec![stmt])
        }
        _ => Err(Error::new_xpand_error("ext attr must be a tuple")),
    }
}
