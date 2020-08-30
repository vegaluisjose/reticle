use crate::backend::arch::ultrascale::reg::*;

impl Default for Expr {
    fn default() -> Self {
        Expr::Ref(String::new())
    }
}
