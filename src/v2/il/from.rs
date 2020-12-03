use crate::v2::il::ast::*;

impl From<ExprTup> for Expr {
    fn from(tup: ExprTup) -> Self {
        Expr::Tup(tup)
    }
}
