use crate::backend::arch::ultrascale::lut::*;

impl Default for Expr {
    fn default() -> Expr {
        Expr::Ref(String::new())
    }
}
