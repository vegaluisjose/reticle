use crate::{Port, PortSet, Prim, ToPrim};
use derive_more::{Deref, DerefMut, Display};

pub const GND: &str = "gnd";

#[derive(Clone, Debug, Default, PartialEq, Eq, Display)]
pub struct GndParam;

#[derive(Clone, Debug, Deref, DerefMut)]
pub struct Gnd(Prim<GndParam>);

#[derive(Clone, Debug, Default)]
struct GndPrim;

impl ToPrim<GndParam> for GndPrim {
    fn to_name(&self) -> String {
        String::from("GND")
    }
    fn to_output(&self) -> PortSet {
        let mut port = PortSet::new();
        port.insert(Port::new("G", 1));
        port
    }
}

impl Default for Gnd {
    fn default() -> Gnd {
        Gnd(GndPrim::default().to_prim())
    }
}
