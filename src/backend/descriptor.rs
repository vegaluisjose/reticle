use crate::lang::ast::*;
use serde::{Deserialize, Serialize};
use std::rc::Rc;
use std::str::FromStr;

// there is probably a way to directly call a default func
// for fields with custom types i.e. https://serde.rs/attr-default.html
// We need something that takes arg, so we can use the from_str trait
// If we solve this, then we can get rid of SerialInstr
// and SerialTarget structs.

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum SerialOp {
    Node(String, Rc<SerialOp>, Rc<SerialOp>),
    Leaf(String, String),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SerialInstr {
    pub name: String,
    pub cost: i32,
    pub loc: String,
    pub ty: String,
    pub output: String,
    pub op: SerialOp,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SerialTarget {
    pub instr: Vec<SerialInstr>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum Op {
    Node(PlacedOp, Rc<Op>, Rc<Op>),
    Leaf(Loc, Id),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Instr {
    pub name: String,
    pub cost: i32,
    pub loc: Loc,
    pub ty: DataType,
    pub output: Id,
    pub op: Op,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Target {
    pub instr: Vec<Instr>,
}

impl Op {
    pub fn from_serial(input: SerialOp) -> Op {
        match input {
            SerialOp::Node(op, left, right) => {
                let lhs = &*left;
                let rhs = &*right;
                Op::Node(
                    PlacedOp::from_str(&op).unwrap(),
                    Rc::new(Op::from_serial(lhs.clone())),
                    Rc::new(Op::from_serial(rhs.clone())),
                )
            }
            SerialOp::Leaf(loc, id) => Op::Leaf(Loc::from_str(&loc).unwrap(), id.to_string()),
        }
    }
}

impl Instr {
    pub fn from_serial(input: SerialInstr) -> Instr {
        Instr {
            name: input.name.to_string(),
            cost: input.cost.clone(),
            loc: Loc::from_str(&input.loc).unwrap(),
            ty: DataType::from_str(&input.ty).unwrap(),
            output: input.output.to_string(),
            op: Op::from_serial(input.op.clone()),
        }
    }
}

impl Target {
    pub fn from_serial(input: SerialTarget) -> Target {
        let mut instr: Vec<Instr> = Vec::new();
        for i in input.instr.iter() {
            instr.push(Instr::from_serial(i.clone()));
        }
        Target {
            instr: instr.to_vec(),
        }
    }
}
