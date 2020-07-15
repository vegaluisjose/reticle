use crate::lang::ast::{PlacedOp, Loc};
use crate::passes::select::instr::*;
use crate::passes::select::pattern::*;
use serde::{Deserialize, Serialize};
use std::rc::Rc;
use std::str::FromStr;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum SerialExpr {
    Op(String),
    Input(String, String),
    Binary(Rc<SerialExpr>, Rc<SerialExpr>, Rc<SerialExpr>),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SerialInstr {
    pub name: String,
    pub cost: i32,
    pub loc: String,
    pub ty: String,
    pub output: String,
    pub expr: SerialExpr,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SerialTarget {
    pub instr: Vec<SerialInstr>,
}

// #[derive(Clone, Debug)]
// pub enum Dexp {
//     Op(PlacedOp),
//     Input(Id, Loc),
//     Binary(Rc<Dexp>, Rc<Dexp>, Rc<Dexp>),
// }

// #[derive(Clone, Debug)]
// pub struct Foo {
//     pub name: String,
//     pub cost: i32,
//     pub loc: Loc,
//     pub ty: DataType,
//     pub output: Id,
//     pub expr: Dexp,
// }

#[derive(Clone, Debug)]
pub struct Target {
    pub cost_map: HashMap<Instr, i32>,
    pub patterns: Vec<Pattern>,
}

// impl Dexp {
//     pub fn from_serial(input: SerialExpr) -> Dexp {
//         match input {
//             SerialExpr::Op(op) => Dexp::Op(PlacedOp::from_str(&op).unwrap()),
//             SerialExpr::Input(id, loc) => Dexp::Input(id.to_string(), Loc::from_str(&loc).unwrap()),
//             SerialExpr::Binary(op, left, right) => {
//                 let o = &*op;
//                 let l = &*left;
//                 let r = &*right;
//                 Dexp::Binary(
//                     Rc::new(Dexp::from_serial(o.clone())),
//                     Rc::new(Dexp::from_serial(l.clone())),
//                     Rc::new(Dexp::from_serial(r.clone())),
//                 )
//             }
//         }
//     }
// }

// impl Foo {
//     pub fn from_serial(input: SerialInstr) -> Foo {
//         Foo {
//             name: input.name.to_string(),
//             cost: input.cost.clone(),
//             loc: Loc::from_str(&input.loc).unwrap(),
//             ty: DataType::from_str(&input.ty).unwrap(),
//             output: input.output.to_string(),
//             expr: Dexp::from_serial(input.expr.clone()),
//         }
//     }
// }

impl Target {
    pub fn from_serial(input: SerialTarget) -> Target {
        let mut cost_map: HashMap<Instr, i32> = HashMap::new();
        let mut patterns: Vec<Pattern> = Vec::new();
        for i in input.instr.iter() {
            match &i.expr {
                SerialExpr::Op(op) => {
                    let op = InstrOp::from_placed_op(&PlacedOp::from_str(&op).unwrap());
                    let loc = InstrLoc::from_loc(&Loc::from_str(&i.loc).unwrap());
                    let ty = InstrTy::from_str(&i.ty).unwrap();
                    let instr = Instr::new(op, ty, loc);
                    if cost_map.contains_key(&instr) {
                        panic!("Error: found a duplicate instr {}");
                    } else {
                        cost_map.insert(instr.clone(), i.cost.clone());
                    }
                },
                SerialExpr::Binary(_, _, _) => {
                    patterns.push(Pattern{name: i.name.to_string(), instr: vec![], cost: i.cost.clone()})
                },
                _ => panic!("Error: input expression alone does not describe one op or pattern"),
            }
        }
        Target {
            cost_map: cost_map.clone(),
            patterns: patterns.to_vec(),
        }
    }
}
