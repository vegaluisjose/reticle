use crate::backend::target::Target;
use std::str::FromStr;

fn spec() -> String {
    let json = r#"
{ "instr":
[
  {"name": "dsp_i8_mul_lut_lut", "cost": 4, "loc": "dsp", "ty": "i8", "output": "y", "expr": ["mul",["i8","lut"],["i8","lut"]]},
  {"name": "dsp_i8_add_lut_lut", "cost": 4, "loc": "dsp", "ty": "i8", "output": "y", "expr": ["add",["i8","lut"],["i8","lut"]]},
  {"name": "dsp_i8_add_reg_mul_lut_lut_lut_lut", "cost": 4, "loc": "dsp", "ty": "i8", "output": "y", "expr": ["add",["reg",["mul",["i8","lut"],["i8","lut"]],["bool", "lut"]],["i8","lut"]]}
]
}"#;
    json.to_string()
}

pub fn target() -> Target {
    Target::from_str(&spec()).unwrap()
}
