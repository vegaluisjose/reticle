use crate::{ParamSet, PortSet, ToPrim};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct ParamValue;

#[derive(Clone, Debug, Default)]
pub struct Dsp;

impl ToPrim<ParamValue> for Dsp {
    fn to_name(&self) -> String {
        String::from("DSP48E2")
    }
    fn to_param(&self) -> ParamSet<ParamValue> {
        ParamSet::new()
    }
    fn to_input(&self) -> PortSet {
        PortSet::new()
    }
    fn to_output(&self) -> PortSet {
        PortSet::new()
    }
}
