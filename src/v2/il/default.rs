use crate::v2::il::ast::*;

impl Default for Sig {
    fn default() -> Self {
        Sig {
            id: String::new(),
            inputs: Expr::Tup(ExprTup::default()),
            outputs: Expr::Tup(ExprTup::default()),
        }
    }
}

impl Default for Expr {
    fn default() -> Self {
        Expr::Tup(ExprTup::default())
    }
}
