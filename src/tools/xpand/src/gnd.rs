use crate::errors::Error;
use crate::to_verilog::{ToVerilogDecl, ToVerilogExpr, ToVerilogInstance, VerilogExprMap};
use crate::vec_expr_try_from_expr;
use prim::ultrascale::gnd::{Gnd, ParamValue, GND};
use prim::{ParamSet, PortSet};
use verilog::ast as vl;
use xir::ast as xir;

impl ToVerilogDecl for Gnd {
    fn to_decl(&self) -> vl::Decl {
        vl::Decl::new_wire(GND, 1)
    }
}

impl ToVerilogExpr for ParamValue {
    fn to_expr(&self) -> vl::Expr {
        vl::Expr::new_ref("")
    }
}

impl ToVerilogInstance<ParamValue> for Gnd {
    fn to_name(&self) -> String {
        format!("_{}", GND)
    }
    fn to_prim(&self) -> String {
        self.name()
    }
    fn to_param_set(&self) -> &ParamSet<ParamValue> {
        self.param()
    }
    fn to_input_set(&self) -> &PortSet {
        self.input()
    }
    fn to_output_set(&self) -> &PortSet {
        self.output()
    }
    fn to_output_map(&self) -> VerilogExprMap {
        let mut map = VerilogExprMap::new();
        for o in self.to_output_set().iter() {
            if o.name().as_str() == "G" {
                map.insert(o.name(), vl::Expr::new_ref(GND));
            } else {
                map.insert(o.name(), vl::Expr::new_ref(""));
            }
        }
        map
    }
}

pub fn from_basc(instr: &xir::InstrBasc) -> Result<Vec<vl::Stmt>, Error> {
    let dst: Vec<vl::Expr> = vec_expr_try_from_expr(instr.dst())?;
    let assign = vl::Parallel::Assign(dst[0].clone(), vl::Expr::new_ref(GND));
    Ok(vec![vl::Stmt::from(assign)])
}
