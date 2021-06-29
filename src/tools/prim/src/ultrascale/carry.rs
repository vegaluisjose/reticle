use crate::{Param, ParamSet, Port, PortSet, Prim, ToPrim};
use derive_more::{Display, From, Deref, DerefMut};

#[derive(Clone, Debug, PartialEq, Eq, Display)]
pub enum CarryType {
    #[display(fmt = "SINGLE_CY8")]
    Dual,
    #[display(fmt = "DUAL_CY4")]
    Single,
}

#[derive(Clone, Debug, From, Eq, Display)]
pub enum CarryParam {
    CarryType(CarryType),
}

#[derive(Clone, Debug, Deref, DerefMut)]
pub struct Carry(Prim<CarryParam>);

#[derive(Clone, Debug, Default)]
struct CarryPrim;

impl PartialEq for CarryParam {
    fn eq(&self, _: &Self) -> bool {
        true
    }
}

impl Default for CarryParam {
    fn default() -> Self {
        CarryParam::from(CarryType::Dual)
    }
}

impl ToPrim<CarryParam> for CarryPrim {
    fn to_name(&self) -> String {
        String::from("CARRY8")
    }
    fn to_param(&self) -> ParamSet<CarryParam> {
        let mut param = ParamSet::new();
        let ty = Param {
            name: "CARRY_TYPE".to_string(),
            width: None,
            value: CarryType::Single.into(),
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
        Carry(CarryPrim::default().to_prim())
    }
}
