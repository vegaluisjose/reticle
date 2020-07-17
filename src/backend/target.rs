// use crate::passes::select::dag_instr::*;
use crate::passes::select::pattern::*;
use serde::{Deserialize, Serialize};
use std::rc::Rc;
use serde_json;
use std::str::FromStr;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum SerialExpr {
    Input(String, String),
    BinOp(String, Rc<SerialExpr>, Rc<SerialExpr>),
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

#[derive(Clone, Debug)]
pub struct Target {
    pub patterns: Vec<Pattern>,
}

impl FromStr for Target {
    type Err = ();
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(Target::from_serial(
            serde_json::from_str(input).expect("Error: parsing json"),
        ))
    }
}

// impl Dexp {
//     pub fn from_serial(input: SerialExpr) -> Dexp {
//         match input {
//             SerialExpr::Op(op) => Dexp::Op(PlacedOp::from_str(&op).unwrap()),
//             SerialExpr::Ref(id, loc) => Dexp::Ref(id.to_string(), Loc::from_str(&loc).unwrap()),
//             SerialExpr::BinOp(op, left, right) => {
//                 let o = &*op;
//                 let l = &*left;
//                 let r = &*right;
//                 Dexp::BinOp(
//                     Rc::new(Dexp::from_serial(o.clone())),
//                     Rc::new(Dexp::from_serial(l.clone())),
//                     Rc::new(Dexp::from_serial(r.clone())),
//                 )
//             }
//         }
//     }
// }

impl Target {
    pub fn from_serial(input: SerialTarget) -> Target {
        let patterns: Vec<Pattern> = Vec::new();
        // for i in input.instr.iter() {
        //     match &i.expr {
        //         // SerialExpr::Op(op) => {
        //             // let op = DagOp::from_placed_op(&PlacedOp::from_str(&op).unwrap());
        //             // let loc = DagLoc::from_loc(&Loc::from_str(&i.loc).unwrap());
        //             // let ty = DagTy::from_str(&i.ty).unwrap();
        //             // let instr = DagInstr::new(op, ty, loc);
        //             // if cost_map.contains_key(&instr) {
        //             //     panic!("Error: found a duplicate instr {}");
        //             // } else {
        //             //     cost_map.insert(instr.clone(), i.cost.clone());
        //             // }
        //         // },
        //         SerialExpr::BinOp(op, lhs, rhs) => {
        //             // patterns.push(Pattern{name: i.name.to_string(), instr: vec![], cost: i.cost.clone()})
        //         },
        //         _ => panic!("Error: reference expression alone does not represent a pattern"),
        //     }
        // }
        Target {
            patterns: patterns.to_vec(),
        }
    }
}
