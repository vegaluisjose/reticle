use crate::backend::descriptor::Target;
use std::str::FromStr;

pub fn target() -> Target {
    let spec = r#"
        { "instr":
          [
            {"name": "dsp_i8_mul", "cost": 4, "loc": "dsp", "ty": "i8", "output": "y", "op": ["mul",["lut","a"],["lut","b"]]},
            {"name": "dsp_i8_add", "cost": 4, "loc": "dsp", "ty": "i8", "output": "y", "op": ["add",["lut","a"],["lut","b"]]},
            {"name": "dsp_i8_add_reg_mul", "cost": 4, "loc": "dsp", "ty": "i8", "output": "y", "op": ["add",["lut","a"],["lut","b"]]}
          ]
        }"#;
    Target::from_str(spec).unwrap()
}
