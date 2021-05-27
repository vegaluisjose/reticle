use crate::errors::Error;
use verilog::ast as vl;

pub trait ToInstance {
    fn to_instance(&self) -> vl::Instance;
    fn to_stmt(&self) -> vl::Stmt;
    fn set_name(&mut self, name: &str);
    fn set_input(&mut self, port: &str, expr: vl::Expr) -> Result<(), Error>;
    fn set_output(&mut self, port: &str, expr: vl::Expr) -> Result<(), Error>;
}
