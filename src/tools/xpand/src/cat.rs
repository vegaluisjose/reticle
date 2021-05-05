use crate::errors::Error;
use crate::vec_expr_try_from_expr;
use verilog::ast as vl;
use xir::ast as xir;

// TODO: add vector type support
pub fn cat_from_basc(instr: &xir::InstrBasc) -> Result<Vec<vl::Stmt>, Error> {
    let term = instr.dst().get_term(0)?;
    if term.is_vector() {
        Err(Error::new_xpand_error("vector ext not supported yet"))
    } else {
        let arg: Vec<vl::Expr> = vec_expr_try_from_expr(instr.arg())?;
        let dst: Vec<vl::Expr> = vec_expr_try_from_expr(instr.dst())?;
        let mut cat = vl::ExprConcat::default();
        for a in arg {
            cat.add_expr(a.clone());
        }
        let assign = vl::Parallel::Assign(dst[0].clone(), vl::Expr::from(cat));
        let stmt = vl::Stmt::from(assign);
        Ok(vec![stmt])
    }
}
