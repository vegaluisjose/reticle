use crate::create_literal;
use crate::loc::{attr_from_loc, Loc};
use prim::{ParamSet, PortSet};
use std::collections::HashMap;
use verilog::ast as vl;

pub type ExprMap = HashMap<String, vl::Expr>;

/// TODO: ToVerilogExpr trait should replace ToExpr trait
pub trait ToVerilogExpr {
    /// emit Verilog expr
    fn to_expr(&self) -> vl::Expr {
        vl::Expr::new_ref("")
    }
}

/// TODO: ToVerilogDecl trait should replace ToDecl trait
pub trait ToVerilogDecl {
    /// emit Verilog expr
    fn to_decl(&self) -> vl::Decl;
}

/// TODO: ToVerilogInstance trait should replace Instance trait
///
/// ToVerilogInstance trait contains methods required to
/// produce block of Verilog code, including at most one
/// instance and multiple statements.
pub trait ToVerilogInstance<T: ToVerilogExpr> {
    /// name of the instance
    fn to_name(&self) -> String;
    /// name of the primitive
    fn to_prim(&self) -> String;
    /// primitive parameter set
    fn to_param_set(&self) -> &ParamSet<T>;
    /// primitive input set
    fn to_input_set(&self) -> &PortSet;
    /// primitive output set
    fn to_output_set(&self) -> &PortSet;
    /// primitive location. Optional, because some primitives
    /// do not require location e.g., VCC or GND
    fn to_loc(&self) -> Option<&Loc> {
        None
    }
    /// parameter map
    fn to_param_map(&self) -> ExprMap {
        let mut map = ExprMap::new();
        for p in self.to_param_set().iter() {
            map.insert(p.name(), p.value().to_expr());
        }
        map
    }
    /// input map
    fn to_input_map(&self) -> ExprMap {
        let mut map = ExprMap::new();
        for i in self.to_input_set().iter() {
            map.insert(i.name(), create_literal(i.width() as u64, 0));
        }
        map
    }
    /// output map
    fn to_output_map(&self) -> ExprMap {
        let mut map = ExprMap::new();
        for o in self.to_output_set().iter() {
            map.insert(o.name(), vl::Expr::new_ref(""));
        }
        map
    }
    /// emit Verilog instance
    fn to_instance(&self) -> vl::Instance {
        let mut inst = vl::Instance::new(&self.to_name(), &self.to_prim());
        let param = self.to_param_map();
        for (p, v) in param {
            inst.add_param(&p, v);
        }
        let input = self.to_input_map();
        for (i, v) in input {
            inst.connect(&i, v);
        }
        let output = self.to_output_map();
        for (o, v) in output {
            inst.connect(&o, v);
        }
        if let Some(loc) = self.to_loc() {
            if loc.is_placed() {
                let attr = attr_from_loc(&loc);
                inst.set_attr(attr);
            }
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
