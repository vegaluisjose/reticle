use std::collections::HashMap;
use verilog::ast as vl;

pub type ExprMap = HashMap<String, vl::Expr>;

/// TODO: ToVerilog trait should replace Instance trait
///
/// ToVerilog trait contains the function required to
/// produce block of Verilog code, containing at most
/// one instance and multiple statements. All methods
/// in this trait are optional
pub trait ToVerilog {
    fn to_name(&self) -> String {
        String::new()
    }
    fn to_prim(&self) -> String {
        String::new()
    }
    fn to_expr(&self) -> vl::Expr {
        vl::Expr::new_str("")
    }
    fn to_param(&self) -> ExprMap {
        ExprMap::new()
    }
    fn to_input(&self) -> ExprMap {
        ExprMap::new()
    }
    fn to_output(&self) -> ExprMap {
        ExprMap::new()
    }
    fn to_instance(&self) -> vl::Instance {
        let param = self.to_param();
        let mut inst = vl::Instance::new(&self.to_name(), &self.to_prim());
        for (p, v) in param {
            inst.add_param(&p, v);
        }
        inst
    }
    fn to_stmt(&self) -> vl::Stmt {
        vl::Stmt::from(self.to_instance())
    }
    fn to_block(&self) -> Vec<vl::Stmt> {
        vec![self.to_stmt()]
    }
}
