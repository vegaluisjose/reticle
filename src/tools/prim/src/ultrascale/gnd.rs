use derive_more::{Deref, DerefMut};
use crate::{Port, PortSet, Prim, ToPrim};

pub const GND: &str = "gnd";

#[derive(Clone, Debug, Default, PartialEq)]
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
        port.insert(Port::new("G", 8));
        port
    }
}
