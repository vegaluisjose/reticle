use crate::errors::Error;
use crate::to_verilog::{ExprMap, ToVerilogDecl, ToVerilogExpr, ToVerilogInstance};
use crate::vec_expr_try_from_expr;
use prim::ultrascale::gnd::{Gnd, GndParam, GND};
use prim::{ParamSet, PortSet};
use verilog::ast as vl;
use xir::ast as xir;

impl ToVerilogDecl for Gnd {
    fn to_decl(&self) -> vl::Decl {
        vl::Decl::new_wire(GND, 1)
    }
}

impl ToVerilogExpr for GndParam {
    fn to_expr(&self) -> vl::Expr {
        vl::Expr::new_ref("")
    }
}

impl ToVerilogInstance<GndParam> for Gnd {
    fn to_name(&self) -> String {
        format!("_{}", GND)
    }
    fn to_prim(&self) -> String {
        self.name()
    }
    fn to_param_set(&self) -> ParamSet<GndParam> {
        self.param().clone()
    }
    fn to_input_set(&self) -> PortSet {
        self.input().clone()
    }
    fn to_output_set(&self) -> PortSet {
        self.output().clone()
    }
    fn to_output_map(&self) -> ExprMap {
        let mut map = ExprMap::new();
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

pub fn gnd_from_basc(instr: &xir::InstrBasc) -> Result<Vec<vl::Stmt>, Error> {
    let dst: Vec<vl::Expr> = vec_expr_try_from_expr(instr.dst())?;
    let assign = vl::Parallel::Assign(dst[0].clone(), vl::Expr::new_ref(GND));
    Ok(vec![vl::Stmt::from(assign)])
}
