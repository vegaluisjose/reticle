use crate::errors::Error;
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

#[derive(Clone, Debug)]
pub struct Lut1 {
    pub name: String,
    pub prim: String,
    pub loc: Loc,
    pub attr: Attr,
    pub input: Input,
    pub output: Output,
}

#[derive(Clone, Debug)]
pub struct Lut2 {
    pub name: String,
    pub prim: String,
    pub loc: Loc,
    pub attr: Attr,
    pub input: Input,
    pub output: Output,
}

#[derive(Clone, Debug)]
pub struct Lut3 {
    pub name: String,
    pub prim: String,
    pub loc: Loc,
    pub attr: Attr,
    pub input: Input,
    pub output: Output,
}

#[derive(Clone, Debug)]
pub struct Lut4 {
    pub name: String,
    pub prim: String,
    pub loc: Loc,
    pub attr: Attr,
    pub input: Input,
    pub output: Output,
}

#[derive(Clone, Debug)]
pub struct Lut5 {
    pub name: String,
    pub prim: String,
    pub loc: Loc,
    pub attr: Attr,
    pub input: Input,
    pub output: Output,
}

#[derive(Clone, Debug)]
pub struct Lut6 {
    pub name: String,
    pub prim: String,
    pub loc: Loc,
    pub attr: Attr,
    pub input: Input,
    pub output: Output,
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

macro_rules! lut_default {
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
            pub fn to_instance(&self) -> vl::Instance {
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
            pub fn to_stmt(&self) -> vl::Stmt {
                vl::Stmt::from(self.to_instance())
            }
            pub fn set_name(&mut self, name: &str) {
                self.name = name.to_string();
            }
            pub fn set_input(&mut self, port: &str, expr: vl::Expr) -> Result<(), Error> {
                if let Some(p) = self.input.connection.get_mut(port) {
                    *p = expr;
                    Ok(())
                } else {
                    let err = format!("input {} do not exist", port);
                    Err(Error::new_xpand_error(&err))
                }
            }
            pub fn set_output(&mut self, port: &str, expr: vl::Expr) -> Result<(), Error> {
                if let Some(p) = self.output.connection.get_mut(port) {
                    *p = expr;
                    Ok(())
                } else {
                    let err = format!("output {} do not exist", port);
                    Err(Error::new_xpand_error(&err))
                }
            }
            pub fn set_init(&mut self, init: u64) {
                self.attr.init = init;
            }
        }
    };
}

lut_default!(Lut1, "LUT1", lut1);
lut_default!(Lut2, "LUT2", lut2);
lut_default!(Lut3, "LUT3", lut3);
lut_default!(Lut4, "LUT4", lut4);
lut_default!(Lut5, "LUT5", lut5);
lut_default!(Lut6, "LUT6", lut6);

lut_impl!(Lut1);
lut_impl!(Lut2);
lut_impl!(Lut3);
lut_impl!(Lut4);
lut_impl!(Lut5);
lut_impl!(Lut6);

pub fn lut2_from_mach(instr: &xir::InstrMach) -> Result<Vec<vl::Stmt>, Error> {
    let mut lut = Lut2::default();
    let name = inst_name_try_from_instr(instr)?;
    let init = instr.attr().get_val(0)?;
    lut.set_init(init as u64);
    lut.set_name(&name);
    let input = ["I0", "I1"];
    let arg: Vec<vl::Expr> = vec_expr_try_from_expr(instr.arg())?;
    for (i, e) in input.iter().zip(arg) {
        lut.set_input(i, e)?;
    }
    let output = ["O"];
    let dst: Vec<vl::Expr> = vec_expr_try_from_expr(instr.dst())?;
    for (o, e) in output.iter().zip(dst) {
        lut.set_output(o, e)?;
    }
    Ok(vec![lut.to_stmt()])
}
