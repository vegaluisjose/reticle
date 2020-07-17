use crate::passes::select::dag_instr::*;
use crate::passes::select::pattern::*;
use serde::{Deserialize, Serialize};
use std::rc::Rc;
use serde_json;
use std::str::FromStr;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum TargetExpr {
    Input(String, String),
    BinOp(String, Rc<TargetExpr>, Rc<TargetExpr>),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TargetInstr {
    pub name: String,
    pub cost: u32,
    pub loc: String,
    pub ty: String,
    pub output: String,
    pub expr: TargetExpr,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TargetDef {
    pub instr: Vec<TargetInstr>,
}

#[derive(Clone, Debug)]
pub struct Target {
    pub patterns: Vec<Pattern>,
}

impl TargetExpr {
    pub fn to_vec_dag_instr(&self, instr: &mut Vec<DagInstr>, op_ty: DagTy, op_loc: DagLoc) {
        match self {
            TargetExpr::Input(_, loc) => {
                let op = DagOp::Inp;
                let inp_loc = DagLoc::from_str(loc).unwrap();
                instr.push(DagInstr::new(op, op_ty.clone(), inp_loc));
            }
            TargetExpr::BinOp(op, lhs, rhs) => {
                let op = DagOp::from_str(op).unwrap();
                instr.push(DagInstr::new(op, op_ty.clone(), op_loc.clone()));
                lhs.to_vec_dag_instr(instr, op_ty.clone(), op_loc.clone());
                rhs.to_vec_dag_instr(instr, op_ty.clone(), op_loc.clone());
            }
        }
    }
}

impl TargetInstr {
    fn to_pattern(&self) -> Pattern {
        let ty = DagTy::from_str(&self.ty).unwrap();
        let loc = DagLoc::from_str(&self.loc).unwrap();
        let mut instr: Vec<DagInstr> = Vec::new();
        self.expr.to_vec_dag_instr(&mut instr, ty, loc);
        Pattern {
            name: self.name.to_string(),
            cost: self.cost.clone(),
            instr: instr.to_vec(),
        }
    }
}

impl Target {
    pub fn from_serial(input: TargetDef) -> Target {
        let mut patterns: Vec<Pattern> = Vec::new();
        for instr in input.instr.iter() {
            patterns.push(instr.to_pattern());
        }
        Target {
            patterns: patterns.to_vec(),
        }
    }
}

impl FromStr for Target {
    type Err = ();
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(Target::from_serial(
            serde_json::from_str(input).expect("Error: parsing json"),
        ))
    }
}