//use crate::errors::Error;
//use crate::loc::Loc;
//use crate::{inst_name_try_from_instr, vec_expr_try_from_expr};
//use prim::ultrascale::carry::{Carry, ParamValue, Ty};
//use xir::ast as xir;

use crate::to_verilog::{ToVerilogExpr, ToVerilogInstance};
use prim::ultrascale::lram::{Lram, LramParam};
use prim::{ParamSet, PortSet};
use verilog::ast as vl;

impl ToVerilogExpr for LramParam {
    fn to_expr(&self) -> vl::Expr {
        match self {
            LramParam::Bool(b) => {
                let v = format!("{}", *b as u32);
                vl::Expr::new_ulit_bin(1, &v)
            }
            LramParam::Bytes(bytes) => {
                let mut v = String::new();
                for b in bytes.iter() {
                    let fmt = format!("{:02X}", b);
                    v.push_str(&fmt);
                }
                let width = (bytes.len() as u32) * 8;
                vl::Expr::new_ulit_hex(width, &v)
            }
        }
    }
}

impl ToVerilogInstance<LramParam> for Lram {
    fn to_name(&self) -> String {
        String::new()
    }
    fn to_prim(&self) -> String {
        self.name()
    }
    fn to_param_set(&self) -> &ParamSet<LramParam> {
        self.param()
    }
    fn to_input_set(&self) -> &PortSet {
        self.input()
    }
    fn to_output_set(&self) -> &PortSet {
        self.output()
    }
}

//#[derive(Clone, Debug)]
//struct CarryAdd {
//    pub prim: Carry,
//    pub instr: xir::InstrMach,
//}
//
//impl CarryAdd {
//    pub fn new(instr: xir::InstrMach) -> Self {
//        CarryAdd {
//            prim: Carry::default(),
//            instr,
//        }
//    }
//}
//
//impl ToVerilogInstance<ParamValue> for CarryAdd {
//    fn to_name(&self) -> String {
//        inst_name_try_from_instr(&self.instr).unwrap()
//    }
//    fn to_prim(&self) -> String {
//        self.prim.name()
//    }
//    fn to_param_set(&self) -> &ParamSet<ParamValue> {
//        self.prim.param()
//    }
//    fn to_input_set(&self) -> &PortSet {
//        self.prim.input()
//    }
//    fn to_output_set(&self) -> &PortSet {
//        self.prim.output()
//    }
//    fn to_loc(&self) -> Option<&Loc> {
//        self.instr.loc()
//    }
//    fn to_input_map(&self) -> ExprMap {
//        let mut map = ExprMap::new();
//        let arg: Vec<vl::Expr> = vec_expr_try_from_expr(self.instr.arg()).unwrap();
//        for p in self.prim.input().iter() {
//            let name = p.name();
//            if name.as_str() == "CI" || name.as_str() == "CI_TOP" {
//                map.insert(name, vl::Expr::new_ulit_bin(1, "0"));
//            } else if name.as_str() == "DI" {
//                map.insert(name, arg[0].clone());
//            } else {
//                map.insert(name, arg[1].clone());
//            }
//        }
//        map
//    }
//    fn to_output_map(&self) -> ExprMap {
//        let mut map = ExprMap::new();
//        let dst: Vec<vl::Expr> = vec_expr_try_from_expr(self.instr.dst()).unwrap();
//        for p in self.prim.output().iter() {
//            let name = p.name();
//            if name.as_str() == "O" {
//                map.insert(name, dst[0].clone());
//            } else {
//                map.insert(name, vl::Expr::new_ref(""));
//            }
//        }
//        map
//    }
//}
//
//pub fn carryadd_from_mach(instr: &xir::InstrMach) -> Result<Vec<vl::Stmt>, Error> {
//    let carry = CarryAdd::new(instr.clone());
//    Ok(carry.to_block())
//}
