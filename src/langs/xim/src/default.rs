use crate::ast::*;

impl Default for Sig {
    fn default() -> Self {
        Sig {
            id: String::new(),
            input: Expr::Tup(ExprTup::default()),
            output: Expr::Tup(ExprTup::default()),
            area: 0,
            perf: 0,
        }
    }
}
