use crate::prim::{ParamSet, PortSet, Prim};

pub trait ToPrim<T> {
    fn to_name(&self) -> String;
    fn to_param(&self) -> ParamSet<T> {
        ParamSet::new()
    }
    fn to_input(&self) -> PortSet {
        PortSet::new()
    }
    fn to_output(&self) -> PortSet {
        PortSet::new()
    }
    fn to_prim(&self) -> Prim<T> {
        Prim {
            name: self.to_name(),
            param: self.to_param(),
            input: self.to_input(),
            output: self.to_output(),
        }
    }
}
