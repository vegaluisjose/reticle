use crate::{ParamSet, PortSet, ToPrim};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct DspParam;

#[derive(Clone, Debug, Default)]
pub struct Dsp;

impl ToPrim<DspParam> for Dsp {
    fn to_name(&self) -> String {
        String::from("DSP48E2")
    }
    fn to_param(&self) -> ParamSet<DspParam> {
        ParamSet::new()
    }
    fn to_input(&self) -> PortSet {
        PortSet::new()
    }
    fn to_output(&self) -> PortSet {
        PortSet::new()
    }
}
