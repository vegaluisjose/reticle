use crate::backend::descriptor::Target;
use std::str::FromStr;

fn spec() -> String {
  let json = r#"
{ "instr":
[
  {"name": "dsp_i8_mul", "cost": 4, "loc": "dsp", "ty": "i8", "output": "y", "expr": "mul"},
  {"name": "dsp_i8_mul_lut_lut", "cost": 4, "loc": "dsp", "ty": "i8", "output": "y", "expr": ["mul",["a","lut"],["b","lut"]]},
  {"name": "dsp_i8_add_lut_lut", "cost": 4, "loc": "dsp", "ty": "i8", "output": "y", "expr": ["add",["a","lut"],["b","lut"]]},
  {"name": "dsp_i8_add_reg_mul_lut_lut_lut_lut", "cost": 4, "loc": "dsp", "ty": "i8", "output": "y", "expr": ["add",["reg",["mul",["a","lut"],["b","lut"]],["en", "lut"]],["c","lut"]]}
]
}"#;
  json.to_string()
}

pub fn target_descriptor() -> Target {
    Target::from_str(&spec()).unwrap()
}
