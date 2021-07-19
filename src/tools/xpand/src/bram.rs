use crate::to_verilog::{ToVerilogExpr, ToVerilogInstance};
use prim::ultrascale::bram::{Bram, ParamValue};
use prim::{ParamSet, PortSet};
use verilog::ast as vl;

impl ToVerilogExpr for ParamValue {
    fn to_expr(&self) -> vl::Expr {
        match self {
            ParamValue::CascadeOrder(v) => vl::Expr::new_str(&v.to_string()),
            ParamValue::ClockDomains(v) => vl::Expr::new_str(&v.to_string()),
            ParamValue::CollisionCheck(v) => vl::Expr::new_str(&v.to_string()),
            ParamValue::FilePath(v) => vl::Expr::new_str(&v.to_string()),
            ParamValue::RstRegPriority(v) => vl::Expr::new_str(&v.to_string()),
            ParamValue::WriteMode(v) => vl::Expr::new_str(&v.to_string()),
            ParamValue::Bool(v) => vl::Expr::new_ulit_bin(1, &format!("{}", *v as i32)),
            ParamValue::BoolNum(v) => vl::Expr::new_int(*v as i32),
            ParamValue::BoolStr(v) => vl::Expr::new_str(&format!("{}", v).to_uppercase()),
            ParamValue::Bytes(width, values) => {
                let mut num = String::new();
                for v in values.iter().rev() {
                    let val = format!("{:02X}", v);
                    num.push_str(&val);
                }
                vl::Expr::new_ulit_hex(*width, &num)
            }
            ParamValue::Num(v) => vl::Expr::new_int(*v as i32),
        }
    }
}

impl ToVerilogInstance<ParamValue> for Bram {
    fn to_name(&self) -> String {
        String::new()
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
}
