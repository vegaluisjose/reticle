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

impl Default for Carry {
    fn default() -> Carry {
        let mut inputs = PortMap::new();
        inputs.insert("vcc".to_string(), Expr::default());
        inputs.insert("gnd".to_string(), Expr::default());
        inputs.insert("a".to_string(), Expr::default());
        inputs.insert("b".to_string(), Expr::default());
        inputs.insert("ci".to_string(), Expr::default());
        let mut outputs = PortMap::new();
        outputs.insert("y".to_string(), Expr::default());
        Carry {
            id: String::new(),
            inputs,
            outputs,
        }
    }
}
