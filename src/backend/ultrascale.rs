use crate::backend::descriptor::Target;
use std::str::FromStr;

pub fn example() {
    let spec = r#"
        { "instr":
          [
            {"name": "dsp_i8_mul", "cost": 4, "loc": "dsp", "ty": "i8", "output": "y", "op": ["mul",["lut","a"],["lut","b"]]},
            {"name": "dsp_i8_add", "cost": 4, "loc": "dsp", "ty": "i8", "output": "y", "op": ["add",["lut","a"],["lut","b"]]},
            {"name": "dsp_i8_add_reg_mul", "cost": 4, "loc": "dsp", "ty": "i8", "output": "y", "op": ["add",["lut","a"],["lut","b"]]}
          ]
        }"#;
    let target_isa = Target::from_str(spec).unwrap();
    for instr in target_isa.instr.iter() {
      println!("{:?}", instr);
    }
}
