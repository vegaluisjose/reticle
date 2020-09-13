use crate::backend::arch::ultrascale::prim::ast::*;

impl Default for Expr {
    fn default() -> Self {
        Expr::Ref(String::new())
    }
}

impl Default for Vcc {
    fn default() -> Vcc {
        let mut outputs = PortMap::new();
        outputs.insert("y".to_string(), Expr::default());
        Vcc {
            id: "VCC".to_string(),
            outputs,
        }
    }
}

impl Default for Gnd {
    fn default() -> Gnd {
        let mut outputs = PortMap::new();
        outputs.insert("y".to_string(), Expr::default());
        Gnd {
            id: "GND".to_string(),
            outputs,
        }
    }
}
