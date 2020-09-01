use crate::backend::arch::ultrascale::prim::ast::*;

impl Default for Expr {
    fn default() -> Self {
        Expr::Ref(String::new())
    }
}

impl Default for Vcc {
    fn default() -> Vcc {
        Vcc {
            id: "VCC".to_string(),
            output: Expr::default(),
        }
    }
}

impl Default for Gnd {
    fn default() -> Gnd {
        Gnd {
            id: "GND".to_string(),
            output: Expr::default(),
        }
    }
}
