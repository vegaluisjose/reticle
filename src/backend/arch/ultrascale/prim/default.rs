use crate::backend::arch::ultrascale::prim::ast::*;

impl Default for Expr {
    fn default() -> Self {
        Expr::Ref(String::new())
    }
}
