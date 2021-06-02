use crate::decl::ToDecl;
use crate::errors::Error;
use crate::instance::ToInstance;
use crate::port::{ConnectionMap, DefaultPort, Port, WidthMap};
use crate::vec_expr_try_from_expr;
use verilog::ast as vl;
use xir::ast as xir;

pub const GND: &str = "gnd";

#[derive(Clone, Debug)]
pub struct Gnd {
    pub name: String,
    pub prim: String,
    pub output: Port,
}

impl DefaultPort for Gnd {
    fn input() -> Port {
        Port::default()
    }
    fn output() -> Port {
        let mut width = WidthMap::new();
        width.insert("G".to_string(), 1);
        let mut connection = ConnectionMap::new();
        for k in width.keys() {
            connection.insert(k.clone(), vl::Expr::new_ref(GND));
        }
        Port { width, connection }
    }
}

impl Default for Gnd {
    fn default() -> Self {
        let name = format!("_{}", GND);
        Gnd {
            name,
            prim: "GND".to_string(),
            output: Gnd::output(),
        }
    }
}

impl ToDecl for Gnd {
    fn to_decl(&self) -> vl::Decl {
        vl::Decl::new_wire(GND, 1)
    }
}

impl ToInstance for Gnd {
    fn to_instance(&self) -> vl::Instance {
        let mut inst = vl::Instance::new(&self.name, &self.prim);
        for (k, v) in self.output.connection.iter() {
            inst.connect(&k, v.clone());
        }
        inst
    }
    fn to_stmt(&self) -> vl::Stmt {
        vl::Stmt::from(self.to_instance())
    }
    fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }
    fn set_input(&mut self, _: &str, _: vl::Expr) -> Result<(), Error> {
        Err(Error::new_xpand_error("GND does not support inputs"))
    }
    fn set_output(&mut self, port: &str, expr: vl::Expr) -> Result<(), Error> {
        if let Some(p) = self.output.connection.get_mut(port) {
            *p = expr;
            Ok(())
        } else {
            let err = format!("output {} do not exist", port);
            Err(Error::new_xpand_error(&err))
        }
    }
}

pub fn gnd_from_basc(instr: &xir::InstrBasc) -> Result<Vec<vl::Stmt>, Error> {
    let dst: Vec<vl::Expr> = vec_expr_try_from_expr(instr.dst())?;
    let assign = vl::Parallel::Assign(dst[0].clone(), vl::Expr::new_ref(GND));
    Ok(vec![vl::Stmt::from(assign)])
}
