use crate::v2::ir::ast::*;

impl Default for Sig {
    fn default() -> Self {
        Sig {
            id: String::new(),
            input: Expr::Tup(ExprTup::default()),
            output: Expr::Tup(ExprTup::default()),
        }
    }
}

impl Default for Expr {
    fn default() -> Self {
        Expr::Tup(ExprTup::default())
    }
}
