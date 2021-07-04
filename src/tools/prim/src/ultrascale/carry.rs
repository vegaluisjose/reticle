use crate::{Param, ParamSet, Port, PortSet, Prim, ToPrim};
use derive_more::{Deref, DerefMut, Display, From};

#[derive(Clone, Debug, PartialEq, Eq, Display)]
pub enum Ty {
    #[display(fmt = "DUAL_CY4")]
    Dual,
    #[display(fmt = "SINGLE_CY8")]
    Single,
}

#[derive(Clone, Debug, From, Eq, Display)]
pub enum ParamValue {
    Ty(Ty),
}

#[derive(Clone, Debug, Deref, DerefMut)]
pub struct Carry(Prim<ParamValue>);

#[derive(Clone, Debug, Default)]
struct CarryPrim;

impl PartialEq for ParamValue {
    fn eq(&self, _: &Self) -> bool {
        true
    }
}

impl ToPrim<ParamValue> for CarryPrim {
    fn to_name(&self) -> String {
        String::from("CARRY8")
    }
    fn to_param(&self) -> ParamSet<ParamValue> {
        let mut param = ParamSet::new();
        let ty = Param {
            name: "CARRY_TYPE".to_string(),
            value: Ty::Single.into(),
        };
        param.insert(ty);
        param
    }
    fn to_input(&self) -> PortSet {
        let mut port = PortSet::new();
        port.insert(Port::new("DI", 8));
        port.insert(Port::new("S", 8));
        port.insert(Port::new("CI", 1));
        port.insert(Port::new("CI_TOP", 1));
        port
    }
    fn to_output(&self) -> PortSet {
        let mut port = PortSet::new();
        port.insert(Port::new("O", 8));
        port.insert(Port::new("CO", 8));
        port
    }
}

impl Default for Carry {
    fn default() -> Carry {
        let carry = CarryPrim;
        Carry(carry.to_prim())
    }
}
