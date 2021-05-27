use crate::decl::ToDecl;
use crate::errors::Error;
use crate::instance::ToInstance;
use crate::port::Output;
use crate::vec_expr_try_from_expr;
use verilog::ast as vl;
use xir::ast as xir;

#[derive(Clone, Debug)]
pub struct Gnd {
    pub name: String,
    pub prim: String,
    pub wire: String,
    pub output: Output,
}

impl Default for Gnd {
    fn default() -> Self {
        let wire = "gnd";
        let name = format!("_{}", wire);
        Gnd {
            name,
            prim: "GND".to_string(),
            wire: wire.to_string(),
            output: Output::gnd(wire),
        }
    }
}

impl Gnd {
    pub fn wire(&self) -> String {
        self.wire.to_string()
    }
}

impl ToDecl for Gnd {
    fn to_decl(&self) -> vl::Decl {
        vl::Decl::new_wire(&self.wire, 1)
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
    let gnd = Gnd::default();
    let wire = gnd.wire();
    let dst: Vec<vl::Expr> = vec_expr_try_from_expr(instr.dst())?;
    let assign = vl::Parallel::Assign(dst[0].clone(), vl::Expr::new_ref(&wire));
    Ok(vec![vl::Stmt::from(assign)])
}
