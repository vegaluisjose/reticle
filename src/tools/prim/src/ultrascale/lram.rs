use crate::{Param, ParamSet, Port, PortSet, Prim, ToPrim};
use derive_more::{Deref, DerefMut, From};
use std::fmt;

#[derive(Clone, Debug, From, Eq)]
pub enum ParamValue {
    Bool(bool),
    Bytes(u32, Vec<u8>),
}

#[derive(Clone, Debug, Deref, DerefMut)]
pub struct Lram(Prim<ParamValue>);

#[derive(Clone, Debug, Default)]
struct LramPrim;

impl PartialEq for ParamValue {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (ParamValue::Bool(_), ParamValue::Bool(_))
            | (ParamValue::Bytes(_, _), ParamValue::Bytes(_, _))
            | (_, _) => false,
        }
    }
}

impl fmt::Display for ParamValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParamValue::Bytes(w, v) => write!(f, "width:{} values:{:?}", w, v),
            _ => write!(f, "{}", self),
        }
    }
}

const LUT: [&str; 8] = ["A", "B", "C", "D", "E", "F", "G", "H"];

impl ToPrim<ParamValue> for LramPrim {
    fn to_name(&self) -> String {
        String::from("RAM64M8")
    }
    fn to_param(&self) -> ParamSet<ParamValue> {
        let mut param = ParamSet::new();
        for l in LUT.iter() {
            let name = format!("INIT_{}", l);
            param.insert(Param {
                name,
                value: (64, vec![0; 8]).into(),
            });
        }
        param.insert(Param {
            name: "IS_WCLK_INVERTED".into(),
            value: false.into(),
        });
        param
    }
    fn to_input(&self) -> PortSet {
        let mut port = PortSet::new();
        for l in &LUT {
            let din = format!("DI{}", l);
            let addr = format!("ADDR{}", l);
            port.insert(Port::new(&din, 1));
            port.insert(Port::new(&addr, 6));
        }
        port.insert(Port::new("WE", 1));
        port.insert(Port::new("WCLK", 1));
        port
    }
    fn to_output(&self) -> PortSet {
        let mut port = PortSet::new();
        for l in &LUT {
            let dout = format!("DO{}", l);
            port.insert(Port::new(&dout, 1));
        }
        port
    }
}

impl Default for Lram {
    fn default() -> Self {
        let ram = LramPrim;
        Lram(ram.to_prim())
    }
}
