use crate::create_literal;
use crate::errors::Error;
use crate::expr::ToExpr;
use crate::instance::ToInstance;
use crate::loc::attr_from_loc;
use crate::loc::{Bel, BelReg, ExprCoord, Loc};
use crate::param::Param;
use crate::port::{ConnectionMap, DefaultPort, Port, WidthMap};
use derive_more::From;
use verilog::ast as vl;

#[derive(Clone, Debug)]
pub enum CascadeOrder {
    None,
    First,
    Last,
    Middle,
}

#[derive(Clone, Debug)]
pub enum ClockDomains {
    Independent,
    Common,
}

#[derive(Clone, Debug, From)]
pub enum ParamValue {
    CascadeOrder(CascadeOrder),
    ClockDomains(ClockDomains),
}

#[derive(Clone, Debug)]
pub struct Ramb18 {
    pub name: String,
    pub prim: String,
    pub param: Param<ParamValue>,
    pub loc: Loc,
    pub input: Port,
    pub output: Port,
}

impl PartialEq for ParamValue {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (ParamValue::CascadeOrder(_), ParamValue::CascadeOrder(_)) => true,
            (ParamValue::ClockDomains(_), ParamValue::ClockDomains(_)) => true,
            (_, _) => false,
        }
    }
}

impl ToExpr for CascadeOrder {
    fn to_expr(&self) -> vl::Expr {
        match self {
            CascadeOrder::None => vl::Expr::new_str("NONE"),
            CascadeOrder::First => vl::Expr::new_str("FIRST"),
            CascadeOrder::Last => vl::Expr::new_str("LAST"),
            CascadeOrder::Middle => vl::Expr::new_str("MIDDLE"),
        }
    }
}

impl ToExpr for ClockDomains {
    fn to_expr(&self) -> vl::Expr {
        match self {
            ClockDomains::Independent => vl::Expr::new_str("INDEPENDENT"),
            ClockDomains::Common => vl::Expr::new_str("COMMON"),
        }
    }
}

impl ToExpr for ParamValue {
    fn to_expr(&self) -> vl::Expr {
        match self {
            ParamValue::CascadeOrder(v) => v.to_expr(),
            ParamValue::ClockDomains(v) => v.to_expr(),
        }
    }
}

impl Default for Param<ParamValue> {
    fn default() -> Self {
        let mut param = Param::<ParamValue>::new();
        param.insert(
            "CASCADE_ORDER_A".to_string(),
            ParamValue::from(CascadeOrder::None),
        );
        param.insert(
            "CASCADE_ORDER_B".to_string(),
            ParamValue::from(CascadeOrder::None),
        );
        param.insert(
            "CLOCK_DOMAINS".to_string(),
            ParamValue::from(ClockDomains::Independent),
        );
        param
    }
}

impl DefaultPort for Ramb18 {
    fn default_input_port() -> Port {
        let mut width = WidthMap::new();
        width.insert("CASDIMUXA".to_string(), 1);
        let mut connection = ConnectionMap::new();
        connection.insert("CASDIMUXA".to_string(), create_literal(1, 0));
        Port { width, connection }
    }
    fn default_output_port() -> Port {
        let mut width = WidthMap::new();
        width.insert("CASDOUTA".to_string(), 16);
        let mut connection = ConnectionMap::new();
        connection.insert("CASDOUTA".to_string(), vl::Expr::new_ref(""));
        Port { width, connection }
    }
}

impl Default for Ramb18 {
    fn default() -> Self {
        let loc = Loc {
            bel: Bel::Reg(BelReg::A),
            x: ExprCoord::default(),
            y: ExprCoord::default(),
        };
        Ramb18 {
            name: String::new(),
            prim: "RAMB18E2".to_string(),
            loc,
            param: Param::<ParamValue>::default(),
            input: Ramb18::default_input_port(),
            output: Ramb18::default_output_port(),
        }
    }
}

impl Ramb18 {
    pub fn set_loc(&mut self, loc: Loc) {
        self.loc = loc;
    }
}

impl ToInstance<ParamValue> for Ramb18 {
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
