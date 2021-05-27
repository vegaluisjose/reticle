use crate::errors::Error;
use crate::instance::ToInstance;
use crate::loc::attr_from_loc;
use crate::loc::{Bel, BelCarry, ExprCoord, Loc};
use crate::port::{Input, Output};
use crate::{inst_name_try_from_instr, vec_expr_try_from_expr};
use verilog::ast as vl;
use xir::ast as xir;

#[derive(Clone, Debug)]
pub enum CarryType {
    Dual,
    Single,
}

#[derive(Clone, Debug)]
pub struct Attr {
    pub ty: CarryType,
}

#[derive(Clone, Debug)]
pub struct Carry {
    pub name: String,
    pub prim: String,
    pub attr: Attr,
    pub loc: Loc,
    pub input: Input,
    pub output: Output,
}

impl Default for Attr {
    fn default() -> Self {
        Attr {
            ty: CarryType::Single,
        }
    }
}

impl Default for Carry {
    fn default() -> Self {
        let loc = Loc {
            bel: Bel::Carry(BelCarry::Carry8),
            x: ExprCoord::default(),
            y: ExprCoord::default(),
        };
        Carry {
            name: String::new(),
            prim: "CARRY8".to_string(),
            loc,
            attr: Attr::default(),
            input: Input::carry(),
            output: Output::carry(),
        }
    }
}

impl ToInstance for Carry {
    fn to_instance(&self) -> vl::Instance {
        let mut inst = vl::Instance::new(&self.name, &self.prim);
        match self.attr.ty {
            CarryType::Single => inst.add_param("CARRY_TYPE", vl::Expr::new_str("SINGLE_CY8")),
            CarryType::Dual => inst.add_param("CARRY_TYPE", vl::Expr::new_str("DUAL_CY4")),
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

pub fn carry_from_carryadd(instr: &xir::InstrMach) -> Result<Vec<vl::Stmt>, Error> {
    let mut carry = Carry::default();
    let name = inst_name_try_from_instr(instr)?;
    carry.set_name(&name);
    let input = ["DI", "S"];
    let arg: Vec<vl::Expr> = vec_expr_try_from_expr(instr.arg())?;
    for (i, e) in input.iter().zip(arg) {
        carry.set_input(i, e)?;
    }
    let input = ["CI", "CI_TOP"];
    for i in &input {
        let zero = vl::Expr::new_ulit_bin(1, "0");
        carry.set_input(i, zero.clone())?;
    }
    let output = ["O"];
    let dst: Vec<vl::Expr> = vec_expr_try_from_expr(instr.dst())?;
    for (o, e) in output.iter().zip(dst) {
        carry.set_output(o, e)?;
    }
    Ok(vec![carry.to_stmt()])
}

pub fn carry_from_mach(instr: &xir::InstrMach) -> Result<Vec<vl::Stmt>, Error> {
    match instr.op() {
        xir::OpMach::CarryAdd => Ok(carry_from_carryadd(instr)?),
        _ => Err(Error::new_xpand_error("unsupported carry instruction")),
    }
}
