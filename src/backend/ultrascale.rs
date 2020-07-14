use serde::{Deserialize, Serialize};
use crate::lang::ast::*;
use std::str::FromStr;
use std::rc::Rc;

// there is probably a way to directly call a default func
// for fields with custom types i.e. https://serde.rs/attr-default.html
// We need something that takes arg, so we can use the from_str trait
// If we solve this, then we can get rid of SerialInstrDescriptor
// and SerialTargetDescriptor structs.

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum SerialTree {
    Node(String, Rc<SerialTree>, Rc<SerialTree>),
    Leaf(String, String),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SerialInstrDescriptor {
    pub name: String,
    pub cost: i32,
    pub loc: String,
    pub ty: String,
    pub output: String,
    pub tree: SerialTree,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SerialTargetDescriptor {
    pub instr: Vec<SerialInstrDescriptor>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum Tree {
    Node(PlacedOp, Rc<Tree>, Rc<Tree>),
    Leaf(Loc, Id),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct InstrDescriptor {
    pub name: String,
    pub cost: i32,
    pub loc: Loc,
    pub ty: DataType,
    pub output: Id,
    pub tree: Tree,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TargetDescriptor {
    pub instr: Vec<InstrDescriptor>,
}

impl Tree {
    pub fn from_serial(input: SerialTree) -> Tree {
        match input {
            SerialTree::Node(op, left, right) => {
                let lhs = &*left;
                let rhs = &*right;
                Tree::Node(PlacedOp::from_str(&op).unwrap(), Rc::new(Tree::from_serial(lhs.clone())), Rc::new(Tree::from_serial(rhs.clone())))
            },
            SerialTree::Leaf(loc, id) => {
                Tree::Leaf(Loc::from_str(&loc).unwrap(), id.to_string())
            },
        }
    }
}

impl InstrDescriptor {
    pub fn from_serial(input: SerialInstrDescriptor) -> InstrDescriptor {
        InstrDescriptor {
            name: input.name.to_string(),
            cost: input.cost.clone(),
            loc: Loc::from_str(&input.loc).unwrap(),
            ty: DataType::from_str(&input.ty).unwrap(),
            output: input.output.to_string(),
            tree: Tree::from_serial(input.tree.clone()),
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
            {"name": "dsp_0", "cost": 4, "loc": "dsp", "ty": "i8", "output": "y", "tree": ["mul",["lut","a"],["lut","b"]]}
          ]
        }"#;
    let target = TargetDescriptor::from_serial(serde_json::from_str(data).expect("Error"));
    println!("This is a target descriptor file:\n{:?}", target);
}