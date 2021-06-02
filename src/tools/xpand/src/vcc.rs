use crate::decl::ToDecl;
use crate::errors::Error;
use crate::instance::ToInstance;
use crate::port::{ConnectionMap, DefaultPort, Port, WidthMap};
use crate::vec_expr_try_from_expr;
use verilog::ast as vl;
use xir::ast as xir;

pub const VCC: &str = "vcc";

#[derive(Clone, Debug)]
pub struct Vcc {
    pub name: String,
    pub prim: String,
    pub output: Port,
}

impl DefaultPort for Vcc {
    fn default_input_port() -> Port {
        Port::default()
    }
    fn default_output_port() -> Port {
        let mut width = WidthMap::new();
        width.insert("G".to_string(), 1);
        let mut connection = ConnectionMap::new();
        for k in width.keys() {
            connection.insert(k.clone(), vl::Expr::new_ref(VCC));
        }
        Port { width, connection }
    }
}

impl Default for Vcc {
    fn default() -> Self {
        let name = format!("_{}", VCC);
        Vcc {
            name,
            prim: "VCC".to_string(),
            output: Vcc::default_output_port(),
        }
    }
}

impl ToDecl for Vcc {
    fn to_decl(&self) -> vl::Decl {
        vl::Decl::new_wire(VCC, 1)
    }
}

impl ToInstance for Vcc {
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
        Err(Error::new_xpand_error("VCC does not support inputs"))
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

pub fn vcc_from_basc(instr: &xir::InstrBasc) -> Result<Vec<vl::Stmt>, Error> {
    let dst: Vec<vl::Expr> = vec_expr_try_from_expr(instr.dst())?;
    let assign = vl::Parallel::Assign(dst[0].clone(), vl::Expr::new_ref(VCC));
    Ok(vec![vl::Stmt::from(assign)])
}
