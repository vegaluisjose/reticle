use crate::errors::Error;
use crate::expr::ToExpr;
use crate::instance::ToInstance;
use crate::loc::attr_from_loc;
use crate::loc::{Bel, BelCarry, ExprCoord, Loc};
use crate::param::Param;
use crate::port::{ConnectionMap, DefaultPort, Port, WidthMap};
use crate::{create_literal, inst_name_try_from_instr, vec_expr_try_from_expr};
use verilog::ast as vl;
use xir::ast as xir;

#[derive(Clone, Debug)]
pub enum CarryType {
    Dual,
    Single,
}

#[derive(Clone, Debug)]
pub enum ParamValue {
    CarryType(CarryType),
}

#[derive(Clone, Debug)]
pub struct Carry {
    pub name: String,
    pub prim: String,
    pub param: Param<ParamValue>,
    pub loc: Loc,
    pub input: Port,
    pub output: Port,
}

// always true because there is only one value type
impl PartialEq for ParamValue {
    fn eq(&self, _: &Self) -> bool {
        true
    }
}

impl ToExpr for CarryType {
    fn to_expr(&self) -> vl::Expr {
        match self {
            CarryType::Single => vl::Expr::new_str("SINGLE_CY8"),
            CarryType::Dual => vl::Expr::new_str("DUAL_CY4"),
        }
    }
}

impl ToExpr for ParamValue {
    fn to_expr(&self) -> vl::Expr {
        match self {
            ParamValue::CarryType(c) => c.to_expr(),
        }
    }
}

impl From<CarryType> for ParamValue {
    fn from(input: CarryType) -> Self {
        ParamValue::CarryType(input)
    }
}

impl Default for Param<ParamValue> {
    fn default() -> Self {
        let mut param = Param::<ParamValue>::new();
        param.insert(
            "CARRY_TYPE".to_string(),
            ParamValue::from(CarryType::Single),
        );
        param
    }
}

impl DefaultPort for Carry {
    fn default_input_port() -> Port {
        let mut width = WidthMap::new();
        width.insert("DI".to_string(), 8);
        width.insert("S".to_string(), 8);
        width.insert("CI".to_string(), 1);
        width.insert("CI_TOP".to_string(), 1);
        let mut connection = ConnectionMap::new();
        for (k, v) in width.iter() {
            connection.insert(k.clone(), create_literal(*v as u64, 0));
        }
        Port { width, connection }
    }
    fn default_output_port() -> Port {
        let mut width = WidthMap::new();
        width.insert("O".to_string(), 8);
        width.insert("CO".to_string(), 8);
        let mut connection = ConnectionMap::new();
        for k in width.keys() {
            connection.insert(k.clone(), vl::Expr::new_ref(""));
        }
        Port { width, connection }
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
            param: Param::<ParamValue>::default(),
            input: Carry::default_input_port(),
            output: Carry::default_output_port(),
        }
    }
}

impl Carry {
    pub fn set_loc(&mut self, loc: Loc) {
        self.loc = loc;
    }
}

impl ToInstance<ParamValue> for Carry {
    fn param(&self) -> &Param<ParamValue> {
        &self.param
    }
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

pub fn carryadd_from_mach(instr: &xir::InstrMach) -> Result<Vec<vl::Stmt>, Error> {
    let mut carry = Carry::default();
    let name = inst_name_try_from_instr(instr)?;
    carry.set_name(&name);
    if let Some(loc) = instr.loc() {
        carry.set_loc(loc.clone());
    }
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
