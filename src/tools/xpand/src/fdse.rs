use crate::errors::Error;
use crate::expr::ToExpr;
use crate::instance::ToInstance;
use crate::loc::attr_from_loc;
use crate::loc::{Bel, BelReg, ExprCoord, Loc};
use crate::param::{Param, ParamMap};
use crate::port::{ConnectionMap, DefaultPort, Port, WidthMap};
use crate::{inst_name_try_from_instr, vec_expr_try_from_expr};
use crate::{CLOCK, RESET};
use verilog::ast as vl;
use xir::ast as xir;

#[derive(Clone, Debug)]
pub enum ParamValue {
    Bool(bool),
}

#[derive(Clone, Debug)]
pub struct Fdse {
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

impl ToExpr for ParamValue {
    fn to_expr(&self) -> vl::Expr {
        match self {
            ParamValue::Bool(b) => {
                let dec = format!("{}", u32::from(*b));
                vl::Expr::new_ulit_bin(1, &dec)
            }
        }
    }
}

impl From<bool> for ParamValue {
    fn from(input: bool) -> Self {
        ParamValue::Bool(input)
    }
}

impl Default for Param<ParamValue> {
    fn default() -> Self {
        let mut map = ParamMap::new();
        map.insert("INIT".to_string(), ParamValue::from(false));
        map.insert("IS_C_INVERTED".to_string(), ParamValue::from(false));
        map.insert("IS_D_INVERTED".to_string(), ParamValue::from(false));
        map.insert("IS_S_INVERTED".to_string(), ParamValue::from(false));
        Param { map }
    }
}

impl DefaultPort for Fdse {
    fn default_input_port() -> Port {
        let mut width = WidthMap::new();
        width.insert("C".to_string(), 1);
        width.insert("S".to_string(), 1);
        width.insert("CE".to_string(), 1);
        width.insert("D".to_string(), 1);
        let mut connection = ConnectionMap::new();
        connection.insert("C".to_string(), vl::Expr::new_ref(CLOCK));
        connection.insert("S".to_string(), vl::Expr::new_ref(RESET));
        connection.insert("CE".to_string(), vl::Expr::new_ulit_hex(1, "0"));
        connection.insert("D".to_string(), vl::Expr::new_ulit_hex(1, "0"));
        Port { width, connection }
    }
    fn default_output_port() -> Port {
        let mut width = WidthMap::new();
        width.insert("Q".to_string(), 1);
        let mut connection = ConnectionMap::new();
        connection.insert("Q".to_string(), vl::Expr::new_ref(""));
        Port { width, connection }
    }
}

impl Default for Fdse {
    fn default() -> Self {
        let loc = Loc {
            bel: Bel::Reg(BelReg::A),
            x: ExprCoord::default(),
            y: ExprCoord::default(),
        };
        Fdse {
            name: String::new(),
            prim: "FDSE".to_string(),
            loc,
            param: Param::<ParamValue>::default(),
            input: Fdse::default_input_port(),
            output: Fdse::default_output_port(),
        }
    }
}

impl Fdse {
    pub fn set_loc(&mut self, loc: Loc) {
        self.loc = loc;
    }
}

impl ToInstance for Fdse {
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

pub fn fdre_from_mach(instr: &xir::InstrMach) -> Result<Vec<vl::Stmt>, Error> {
    let mut fdse = Fdse::default();
    let name = inst_name_try_from_instr(instr)?;
    fdse.set_name(&name);
    if let Some(loc) = instr.loc() {
        fdse.set_loc(loc.clone());
    }
    let input = ["D", "CE"];
    let arg: Vec<vl::Expr> = vec_expr_try_from_expr(instr.arg())?;
    for (i, e) in input.iter().zip(arg) {
        fdse.set_input(i, e)?;
    }
    let output = ["Q"];
    let dst: Vec<vl::Expr> = vec_expr_try_from_expr(instr.dst())?;
    for (o, e) in output.iter().zip(dst) {
        fdse.set_output(o, e)?;
    }
    Ok(vec![fdse.to_stmt()])
}
