use crate::backend::target::descriptor::Descriptor;
use crate::backend::target::Target;
use std::str::FromStr;

pub struct Ultrascale {
    pub spec: String,
}

impl Ultrascale {
    pub fn new() -> Ultrascale {
        let json = r#"
        { "isa":
        [
          {"name": "dsp_i8_mul_lut_lut", "cost": 4, "loc": "dsp", "ty": "i8", "area": 1, "expr": ["mul",["i8","lut"],["i8","lut"]]},
          {"name": "dsp_i8_add_lut_lut", "cost": 4, "loc": "dsp", "ty": "i8", "area": 1, "expr": ["add",["i8","lut"],["i8","lut"]]},
          {"name": "dsp_i8_add_reg_mul_lut_lut_lut_lut", "cost": 4, "loc": "dsp", "ty": "i8", "area": 1, "expr": ["add",["reg",["mul",["i8","lut"],["i8","lut"]],["bool", "lut"]],["i8","lut"]]}
        ]
        }"#;
        Ultrascale {
            spec: json.to_string(),
        }
    }
}

impl Target for Ultrascale {
    fn to_descriptor(&self) -> Descriptor {
        Descriptor::from_str(&self.spec).unwrap()
    }
}