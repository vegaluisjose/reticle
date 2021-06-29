use crate::create_literal;
use prim::{ParamSet, PortSet};
use std::collections::HashMap;
use verilog::ast as vl;

pub type ExprMap = HashMap<String, vl::Expr>;

/// TODO: ToVerilogExpr trait should replace ToExpr trait
pub trait ToVerilogExpr {
    /// emit Verilog expr
    fn to_expr(&self) -> vl::Expr;
}

/// TODO: ToVerilogInstance trait should replace Instance trait
///
/// ToVerilogInstance trait contains the function required to
/// produce block of Verilog code, containing at most
/// one instance and multiple statements.
pub trait ToVerilogInstance<T: ToVerilogExpr> {
    /// name of the instance
    fn to_name(&self) -> String;
    /// name of the primitive
    fn to_prim(&self) -> String;
    /// primitive parameter set
    fn to_param_set(&self) -> ParamSet<T>;
    /// primitive input set
    fn to_input_set(&self) -> PortSet;
    /// primitive output set
    fn to_output_set(&self) -> PortSet;
    /// emit Verilog instance
    fn to_instance(&self) -> vl::Instance {
        let mut inst = vl::Instance::new(&self.to_name(), &self.to_prim());
        for p in self.to_param_set().iter() {
            inst.add_param(&p.name(), p.value().to_expr());
        }
        for i in self.to_input_set().iter() {
            inst.connect(&i.name, create_literal(i.width() as u64, 0));
        }
        for o in self.to_output_set().iter() {
            inst.connect(&o.name(), vl::Expr::new_ref(""));
        }
        inst
    }
    /// emit Verilog stmt
    fn to_stmt(&self) -> vl::Stmt {
        vl::Stmt::from(self.to_instance())
    }
    /// emit Verilog block
    fn to_block(&self) -> Vec<vl::Stmt> {
        vec![self.to_stmt()]
    }
}
