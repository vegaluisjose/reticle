use std::collections::HashMap;
use verilog::ast as vl;

pub type ConnectionMap = HashMap<String, vl::Expr>;
pub type WidthMap = HashMap<String, u32>;

#[derive(Clone, Debug, Default)]
pub struct Port {
    pub width: WidthMap,
    pub connection: ConnectionMap,
}

#[derive(Clone, Debug)]
pub struct Input {
    pub width: WidthMap,
    pub connection: ConnectionMap,
}

#[derive(Clone, Debug)]
pub struct Output {
    pub width: WidthMap,
    pub connection: ConnectionMap,
}

pub trait DefaultPort {
    // implement default inputs
    fn input() -> Port;
    // implement default outputs
    fn output() -> Port;
}

impl Port {
    pub fn get_width(&self, port: &str) -> Option<&u32> {
        self.width.get(port)
    }
}

impl Input {
    pub fn lut1() -> Self {
        let mut width = WidthMap::new();
        width.insert("I0".to_string(), 1);
        let mut connection = ConnectionMap::new();
        for (k, v) in width.iter() {
            connection.insert(k.clone(), vl::Expr::new_ulit_hex(*v, "0"));
        }
        Input { width, connection }
    }
    pub fn lut2() -> Self {
        let mut width = WidthMap::new();
        width.insert("I0".to_string(), 1);
        width.insert("I1".to_string(), 1);
        let mut connection = ConnectionMap::new();
        for (k, v) in width.iter() {
            connection.insert(k.clone(), vl::Expr::new_ulit_hex(*v, "0"));
        }
        Input { width, connection }
    }
    pub fn lut3() -> Self {
        let mut width = WidthMap::new();
        width.insert("I0".to_string(), 1);
        width.insert("I1".to_string(), 1);
        width.insert("I2".to_string(), 1);
        let mut connection = ConnectionMap::new();
        for (k, v) in width.iter() {
            connection.insert(k.clone(), vl::Expr::new_ulit_hex(*v, "0"));
        }
        Input { width, connection }
    }
    pub fn lut4() -> Self {
        let mut width = WidthMap::new();
        width.insert("I0".to_string(), 1);
        width.insert("I1".to_string(), 1);
        width.insert("I2".to_string(), 1);
        width.insert("I3".to_string(), 1);
        let mut connection = ConnectionMap::new();
        for (k, v) in width.iter() {
            connection.insert(k.clone(), vl::Expr::new_ulit_hex(*v, "0"));
        }
        Input { width, connection }
    }
    pub fn lut5() -> Self {
        let mut width = WidthMap::new();
        width.insert("I0".to_string(), 1);
        width.insert("I1".to_string(), 1);
        width.insert("I2".to_string(), 1);
        width.insert("I3".to_string(), 1);
        width.insert("I4".to_string(), 1);
        let mut connection = ConnectionMap::new();
        for (k, v) in width.iter() {
            connection.insert(k.clone(), vl::Expr::new_ulit_hex(*v, "0"));
        }
        Input { width, connection }
    }
    pub fn lut6() -> Self {
        let mut width = WidthMap::new();
        width.insert("I0".to_string(), 1);
        width.insert("I1".to_string(), 1);
        width.insert("I2".to_string(), 1);
        width.insert("I3".to_string(), 1);
        width.insert("I4".to_string(), 1);
        width.insert("I5".to_string(), 1);
        let mut connection = ConnectionMap::new();
        for (k, v) in width.iter() {
            connection.insert(k.clone(), vl::Expr::new_ulit_hex(*v, "0"));
        }
        Input { width, connection }
    }
    pub fn carry() -> Self {
        let mut width = WidthMap::new();
        width.insert("DI".to_string(), 8);
        width.insert("S".to_string(), 8);
        width.insert("CI".to_string(), 1);
        width.insert("CI_TOP".to_string(), 1);
        let mut connection = ConnectionMap::new();
        for (k, v) in width.iter() {
            connection.insert(k.clone(), vl::Expr::new_ulit_hex(*v, "0"));
        }
        Input { width, connection }
    }
    pub fn dsp() -> Self {
        let mut width = WidthMap::new();
        width.insert("ACIN".to_string(), 30);
        width.insert("BCIN".to_string(), 18);
        width.insert("CARRYCASCIN".to_string(), 1);
        width.insert("MULTSIGNIN".to_string(), 1);
        width.insert("PCIN".to_string(), 48);
        width.insert("ALUMODE".to_string(), 4);
        width.insert("CARRYINSEL".to_string(), 3);
        width.insert("CLK".to_string(), 1);
        width.insert("INMODE".to_string(), 5);
        width.insert("OPMODE".to_string(), 9);
        width.insert("A".to_string(), 30);
        width.insert("B".to_string(), 18);
        width.insert("C".to_string(), 48);
        width.insert("CARRYIN".to_string(), 1);
        width.insert("D".to_string(), 27);
        width.insert("CEA1".to_string(), 1);
        width.insert("CEA2".to_string(), 1);
        width.insert("CEAD".to_string(), 1);
        width.insert("CEALUMODE".to_string(), 1);
        width.insert("CEB1".to_string(), 1);
        width.insert("CEB2".to_string(), 1);
        width.insert("CEC".to_string(), 1);
        width.insert("CECARRYIN".to_string(), 1);
        width.insert("CECTRL".to_string(), 1);
        width.insert("CED".to_string(), 1);
        width.insert("CEINMODE".to_string(), 1);
        width.insert("CEM".to_string(), 1);
        width.insert("CEP".to_string(), 1);
        width.insert("RSTA".to_string(), 1);
        width.insert("RSTALLCARRYIN".to_string(), 1);
        width.insert("RSTALUMODE".to_string(), 1);
        width.insert("RSTB".to_string(), 1);
        width.insert("RSTC".to_string(), 1);
        width.insert("RSTCTRL".to_string(), 1);
        width.insert("RSTD".to_string(), 1);
        width.insert("RSTINMODE".to_string(), 1);
        width.insert("RSTM".to_string(), 1);
        width.insert("RSTP".to_string(), 1);
        let mut connection = ConnectionMap::new();
        for (k, v) in width.iter() {
            connection.insert(k.clone(), vl::Expr::new_ulit_hex(*v, "0"));
        }
        Input { width, connection }
    }
    pub fn get_width(&self, port: &str) -> Option<&u32> {
        self.width.get(port)
    }
}

impl Output {
    pub fn gnd(name: &str) -> Self {
        let mut width = WidthMap::new();
        width.insert("G".to_string(), 1);
        let mut connection = ConnectionMap::new();
        for k in width.keys() {
            connection.insert(k.clone(), vl::Expr::new_ref(name));
        }
        Output { width, connection }
    }
    pub fn vcc(name: &str) -> Self {
        let mut width = WidthMap::new();
        width.insert("P".to_string(), 1);
        let mut connection = ConnectionMap::new();
        for k in width.keys() {
            connection.insert(k.clone(), vl::Expr::new_ref(name));
        }
        Output { width, connection }
    }
    pub fn dsp() -> Self {
        let mut width = WidthMap::new();
        width.insert("ACOUT".to_string(), 30);
        width.insert("BCOUT".to_string(), 18);
        width.insert("CARRYCASCOUT".to_string(), 1);
        width.insert("MULTSIGNOUT".to_string(), 1);
        width.insert("PCOUT".to_string(), 48);
        width.insert("OVERFLOW".to_string(), 1);
        width.insert("PATTERNBDETECT".to_string(), 1);
        width.insert("PATTERNDETECT".to_string(), 1);
        width.insert("UNDERFLOW".to_string(), 1);
        width.insert("CARRYOUT".to_string(), 4);
        width.insert("P".to_string(), 48);
        width.insert("XOROUT".to_string(), 8);
        let mut connection = ConnectionMap::new();
        for k in width.keys() {
            connection.insert(k.clone(), vl::Expr::new_ref(""));
        }
        Output { width, connection }
    }
    pub fn lut() -> Self {
        let mut width = WidthMap::new();
        width.insert("O".to_string(), 1);
        let mut connection = ConnectionMap::new();
        for k in width.keys() {
            connection.insert(k.clone(), vl::Expr::new_ref(""));
        }
        Output { width, connection }
    }
    pub fn carry() -> Self {
        let mut width = WidthMap::new();
        width.insert("O".to_string(), 8);
        width.insert("CO".to_string(), 8);
        let mut connection = ConnectionMap::new();
        for k in width.keys() {
            connection.insert(k.clone(), vl::Expr::new_ref(""));
        }
        Output { width, connection }
    }
    pub fn get_width(&self, port: &str) -> Option<&u32> {
        self.width.get(port)
    }
}
