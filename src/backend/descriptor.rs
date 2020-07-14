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
pub enum SerialDexp {
    Op(String),
    Input(String, String),
    Binary(Rc<SerialDexp>, Rc<SerialDexp>, Rc<SerialDexp>),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SerialInstr {
    pub name: String,
    pub cost: i32,
    pub loc: String,
    pub ty: String,
    pub output: String,
    pub expr: SerialDexp,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SerialTarget {
    pub instr: Vec<SerialInstr>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum Dexp {
    Op(PlacedOp),
    Input(Id, Loc),
    Binary(Rc<Dexp>, Rc<Dexp>, Rc<Dexp>),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Instr {
    pub name: String,
    pub cost: i32,
    pub loc: Loc,
    pub ty: DataType,
    pub output: Id,
    pub expr: Dexp,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Target {
    pub instr: Vec<Instr>,
}

impl Dexp {
    pub fn from_serial(input: SerialDexp) -> Dexp {
        match input {
            SerialDexp::Op(op) => Dexp::Op(PlacedOp::from_str(&op).unwrap()),
            SerialDexp::Input(id, loc) => Dexp::Input(id.to_string(), Loc::from_str(&loc).unwrap()),
            SerialDexp::Binary(op, left, right) => {
                let o = &*op;
                let l = &*left;
                let r = &*right;
                Dexp::Binary(
                    Rc::new(Dexp::from_serial(o.clone())),
                    Rc::new(Dexp::from_serial(l.clone())),
                    Rc::new(Dexp::from_serial(r.clone())),
                )
            }
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
            expr: Dexp::from_serial(input.expr.clone()),
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
