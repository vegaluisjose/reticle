use crate::errors::Error;
use crate::instance::ToInstance;
use crate::loc::attr_from_loc;
use crate::loc::{Bel, BelLut, ExprCoord, Loc};
use crate::port::{Input, Output};
use crate::{inst_name_try_from_instr, vec_expr_try_from_expr};
use verilog::ast as vl;
use xir::ast as xir;

#[derive(Clone, Debug)]
pub struct Attr {
    pub init: u64,
    pub width: u32,
}

impl Default for Attr {
    fn default() -> Self {
        Attr { init: 0, width: 0 }
    }
}

impl Attr {
    pub fn lut1() -> Self {
        Attr { init: 0, width: 2 }
    }
    pub fn lut2() -> Self {
        Attr { init: 0, width: 4 }
    }
    pub fn lut3() -> Self {
        Attr { init: 0, width: 8 }
    }
    pub fn lut4() -> Self {
        Attr { init: 0, width: 16 }
    }
    pub fn lut5() -> Self {
        Attr { init: 0, width: 32 }
    }
    pub fn lut6() -> Self {
        Attr { init: 0, width: 64 }
    }
}

macro_rules! lut {
    ($ty:tt) => {
        #[derive(Clone, Debug)]
        pub struct $ty {
            pub name: String,
            pub prim: String,
            pub loc: Loc,
            pub attr: Attr,
            pub input: Input,
            pub output: Output,
        }
    };
}

macro_rules! lut_impl_default {
    ($ty:tt, $prim:tt, $default:tt) => {
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
                    attr: Attr::$default(),
                    input: Input::$default(),
                    output: Output::lut(),
                }
            }
        }
    };
}

macro_rules! lut_impl {
    ($ty:tt) => {
        impl $ty {
            pub fn set_init(&mut self, init: u64) {
                self.attr.init = init;
            }
            pub fn set_loc(&mut self, loc: Loc) {
                self.loc = loc;
            }
        }
    };
}

macro_rules! lut_impl_instance {
    ($ty:tt) => {
        impl ToInstance for $ty {
            fn to_instance(&self) -> vl::Instance {
                let mut inst = vl::Instance::new(&self.name, &self.prim);
                let init = format!("{:X}", self.attr.init);
                inst.add_param("INIT", vl::Expr::new_ulit_hex(self.attr.width, &init));
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
    };
}

macro_rules! fn_lut_from_mach {
    ($fn:ident, $ty:tt, $input:expr) => {
        pub fn $fn(instr: &xir::InstrMach) -> Result<Vec<vl::Stmt>, Error> {
            let mut lut = $ty::default();
            let name = inst_name_try_from_instr(instr)?;
            lut.set_name(&name);
            if let Some(loc) = instr.loc() {
                lut.set_loc(loc.clone());
            }
            let init = instr.attr().get_val(0)?;
            lut.set_init(init as u64);
            let arg: Vec<vl::Expr> = vec_expr_try_from_expr(instr.arg())?;
            for (i, e) in $input.iter().zip(arg) {
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

lut!(Lut1);
lut!(Lut2);
lut!(Lut3);
lut!(Lut4);
lut!(Lut5);
lut!(Lut6);

lut_impl_default!(Lut1, "LUT1", lut1);
lut_impl_default!(Lut2, "LUT2", lut2);
lut_impl_default!(Lut3, "LUT3", lut3);
lut_impl_default!(Lut4, "LUT4", lut4);
lut_impl_default!(Lut5, "LUT5", lut5);
lut_impl_default!(Lut6, "LUT6", lut6);

lut_impl!(Lut1);
lut_impl!(Lut2);
lut_impl!(Lut3);
lut_impl!(Lut4);
lut_impl!(Lut5);
lut_impl!(Lut6);

lut_impl_instance!(Lut1);
lut_impl_instance!(Lut2);
lut_impl_instance!(Lut3);
lut_impl_instance!(Lut4);
lut_impl_instance!(Lut5);
lut_impl_instance!(Lut6);

fn_lut_from_mach!(lut1_from_mach, Lut1, ["I0"]);
fn_lut_from_mach!(lut2_from_mach, Lut2, ["I0", "I1"]);
fn_lut_from_mach!(lut3_from_mach, Lut3, ["I0", "I1", "I2"]);
fn_lut_from_mach!(lut4_from_mach, Lut4, ["I0", "I1", "I2", "I3"]);
fn_lut_from_mach!(lut5_from_mach, Lut5, ["I0", "I1", "I2", "I3", "I4"]);
fn_lut_from_mach!(lut6_from_mach, Lut6, ["I0", "I1", "I2", "I3", "I4", "I5"]);
