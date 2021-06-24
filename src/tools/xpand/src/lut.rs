use crate::errors::Error;
use crate::expr::ToExpr;
use crate::instance::ToInstance;
use crate::loc::attr_from_loc;
use crate::loc::{Bel, BelLut, ExprCoord, Loc};
use crate::param::Param;
use crate::port::{ConnectionMap, DefaultPort, Port, WidthMap};
use crate::{create_literal, inst_name_try_from_instr, vec_expr_try_from_expr};
use verilog::ast as vl;
use xir::ast as xir;

macro_rules! lut {
    ($fn:ident, $ty:tt, $val:tt, $prim:expr, $width:expr, $inputs:expr) => {
        #[derive(Clone, Debug)]
        pub enum $val {
            Init(u64),
        }

        #[derive(Clone, Debug)]
        pub struct $ty {
            pub name: String,
            pub prim: String,
            pub loc: Loc,
            pub param: Param<$val>,
            pub input: Port,
            pub output: Port,
        }

        // always true because there is only one value type
        impl PartialEq for $val {
            fn eq(&self, _: &Self) -> bool {
                true
            }
        }

        impl ToExpr for $val {
            fn to_expr(&self) -> vl::Expr {
                match self {
                    $val::Init(v) => {
                        let s = format!("{:x}", *v);
                        vl::Expr::new_ulit_hex($width, &s)
                    }
                }
            }
        }

        impl From<u64> for $val {
            fn from(input: u64) -> Self {
                $val::Init(input)
            }
        }

        impl Default for Param<$val> {
            fn default() -> Self {
                let mut param = Param::<$val>::new();
                param.insert("INIT".to_string(), $val::from(0));
                param
            }
        }

        impl DefaultPort for $ty {
            fn default_input_port() -> Port {
                let mut width = WidthMap::new();
                for i in $inputs.iter() {
                    width.insert(i.to_string(), 1);
                }
                let mut connection = ConnectionMap::new();
                for (k, v) in width.iter() {
                    connection.insert(k.clone(), create_literal(*v as u64, 0));
                }
                Port { width, connection }
            }
            fn default_output_port() -> Port {
                let mut width = WidthMap::new();
                width.insert("O".to_string(), 1);
                let mut connection = ConnectionMap::new();
                for k in width.keys() {
                    connection.insert(k.clone(), vl::Expr::new_ref(""));
                }
                Port { width, connection }
            }
        }

        impl Default for $ty {
            fn default() -> Self {
                let loc = Loc {
                    bel: Bel::Lut(BelLut::A6),
                    x: ExprCoord::default(),
                    y: ExprCoord::default(),
                };
                $ty {
                    name: String::new(),
                    prim: $prim.to_string(),
                    loc,
                    param: Param::<$val>::default(),
                    input: $ty::default_input_port(),
                    output: $ty::default_output_port(),
                }
            }
        }

        impl ToInstance for $ty {
            fn to_instance(&self) -> vl::Instance {
                let mut inst = vl::Instance::new(&self.name, &self.prim);
                for (k, v) in self.param.param() {
                    let expr: vl::Expr = v.clone().to_expr();
                    inst.add_param(k, expr);
                }
                for (k, v) in self.input.connection.iter() {
                    inst.connect(&k, v.clone());
                }
                for (k, v) in self.output.connection.iter() {
                    inst.connect(&k, v.clone());
                }
                if self.loc.is_placed() {
                    let attr = attr_from_loc(&self.loc);
                    inst.set_attr(attr);
                }
                inst
            }
            fn to_stmt(&self) -> vl::Stmt {
                vl::Stmt::from(self.to_instance())
            }
            fn set_name(&mut self, name: &str) {
                self.name = name.to_string();
            }
            fn set_input(&mut self, port: &str, expr: vl::Expr) -> Result<(), Error> {
                if let Some(p) = self.input.connection.get_mut(port) {
                    *p = expr;
                    Ok(())
                } else {
                    let err = format!("input {} do not exist", port);
                    Err(Error::new_xpand_error(&err))
                }
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

        impl $ty {
            pub fn set_loc(&mut self, loc: Loc) {
                self.loc = loc;
            }
            pub fn set_param<P>(&mut self, name: &str, value: P) -> Result<(), Error>
            where
                P: Into<$val>,
            {
                self.param.set_param(name, value.into())?;
                Ok(())
            }
        }

        pub fn $fn(instr: &xir::InstrMach) -> Result<Vec<vl::Stmt>, Error> {
            let mut lut = $ty::default();
            let name = inst_name_try_from_instr(instr)?;
            lut.set_name(&name);
            if let Some(loc) = instr.loc() {
                lut.set_loc(loc.clone());
            }
            let init = instr.attr().get_val(0)?;
            lut.set_param("INIT", init as u64)?;
            let arg: Vec<vl::Expr> = vec_expr_try_from_expr(instr.arg())?;
            for (i, e) in $inputs.iter().zip(arg) {
                lut.set_input(i, e)?;
            }
            let output = ["O"];
            let dst: Vec<vl::Expr> = vec_expr_try_from_expr(instr.dst())?;
            for (o, e) in output.iter().zip(dst) {
                lut.set_output(o, e)?;
            }
            Ok(vec![lut.to_stmt()])
        }
    };
}

lut!(lut1_from_mach, Lut1, Lut1ParamVal, "LUT1", 2, ["I0"]);
lut!(lut2_from_mach, Lut2, Lut2ParamVal, "LUT2", 4, ["I0", "I1"]);
lut!(
    lut3_from_mach,
    Lut3,
    Lut3ParamVal,
    "LUT3",
    8,
    ["I0", "I1", "I2"]
);
lut!(
    lut4_from_mach,
    Lut4,
    Lut4ParamVal,
    "LUT4",
    16,
    ["I0", "I1", "I2", "I3"]
);
lut!(
    lut5_from_mach,
    Lut5,
    Lut5ParamVal,
    "LUT5",
    32,
    ["I0", "I1", "I2", "I3", "I4"]
);
lut!(
    lut6_from_mach,
    Lut6,
    Lut6ParamVal,
    "LUT6",
    64,
    ["I0", "I1", "I2", "I3", "I4", "I5"]
);
