use crate::{Port, PortSet, Prim, ToPrim};
use derive_more::{Deref, DerefMut, Display};

pub const VCC: &str = "vcc";

#[derive(Clone, Debug, Default, PartialEq, Eq, Display)]
pub struct ParamValue;

#[derive(Clone, Debug, Deref, DerefMut)]
pub struct Vcc(Prim<ParamValue>);

#[derive(Clone, Debug, Default)]
struct VccPrim;

impl ToPrim<ParamValue> for VccPrim {
    fn to_name(&self) -> String {
        String::from("VCC")
    }
    fn to_output(&self) -> PortSet {
        let mut port = PortSet::new();
        port.insert(Port::new("P", 1));
        port
    }
}

impl Default for Vcc {
    fn default() -> Vcc {
        Vcc(VccPrim::default().to_prim())
    }
}
