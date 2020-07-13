use serde::{Deserialize, Serialize};
use crate::lang::ast::*;
use std::str::FromStr;

// there is probably a way to directly call a default func
// for fields with custom types i.e. https://serde.rs/attr-default.html
// We need something that takes arg, so we can use the from_str trait
// If we solve this, then we can get rid of SerialInstrDescriptor
// and SerialTargetDescriptor structs.

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SerialInstrDescriptor {
    pub cost: i32,
    pub loc: String,
    pub ty: String,
    pub output: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SerialTargetDescriptor {
    pub instr: Vec<SerialInstrDescriptor>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct InstrDescriptor {
    pub cost: i32,
    pub loc: Loc,
    pub ty: DataType,
    pub output: Id,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TargetDescriptor {
    pub instr: Vec<InstrDescriptor>,
}

impl InstrDescriptor {
    pub fn from_serial(input: SerialInstrDescriptor) -> InstrDescriptor {
        InstrDescriptor {
            cost: input.cost.clone(),
            loc: Loc::from_str(&input.loc).unwrap(),
            ty: DataType::from_str(&input.ty).unwrap(),
            output: input.output.to_string(),
        }
    }
}

impl TargetDescriptor {
    pub fn from_serial(input: SerialTargetDescriptor) -> TargetDescriptor {
        let mut instr: Vec<InstrDescriptor> = Vec::new();
        for i in input.instr.iter() {
            instr.push(InstrDescriptor::from_serial(i.clone()));
        }
        TargetDescriptor { instr: instr.to_vec() }
    }
}

pub fn example() {
    let data = r#"
        { "instr":
          [
            {"cost": -1, "loc": "dsp", "ty": "i8", "output": "y"}
          ]
        }"#;
    let target = TargetDescriptor::from_serial(serde_json::from_str(data).expect("Error"));
    println!("This is a target descriptor file:\n{:?}", target);
}
