use crate::passes::select::instr;
use serde::{Deserialize, Serialize};
use serde_json;
use std::rc::Rc;
use std::str::FromStr;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum Expr {
    Input(String, String),
    BinOp(String, Rc<Expr>, Rc<Expr>),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Instr {
    pub name: String,
    pub cost: u32,
    pub loc: String,
    pub ty: String,
    pub output: String,
    pub expr: Expr,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Def {
    pub instr: Vec<Instr>,
}

#[derive(Clone, Debug)]
pub struct Target {
    pub patterns: Vec<instr::Pattern>,
}

impl Expr {
    pub fn to_instr_mut(
        &self,
        instr: &mut Vec<instr::Instr>,
        op_ty: instr::Ty,
        op_loc: instr::Loc,
    ) {
        match self {
            Expr::Input(_, loc) => {
                let op = instr::Op::Inp;
                let inp_loc = instr::Loc::from_str(loc).unwrap();
                instr.push(instr::Instr::new(op, op_ty.clone(), inp_loc));
            }
            Expr::BinOp(op, lhs, rhs) => {
                let op = instr::Op::from_str(op).unwrap();
                instr.push(instr::Instr::new(op, op_ty.clone(), op_loc.clone()));
                lhs.to_instr_mut(instr, op_ty.clone(), op_loc.clone());
                rhs.to_instr_mut(instr, op_ty.clone(), op_loc.clone());
            }
        }
    }
}

impl Instr {
    fn to_pattern(&self) -> instr::Pattern {
        let ty = instr::Ty::from_str(&self.ty).unwrap();
        let loc = instr::Loc::from_str(&self.loc).unwrap();
        let mut instr: Vec<instr::Instr> = Vec::new();
        self.expr.to_instr_mut(&mut instr, ty, loc);
        instr::Pattern {
            name: self.name.to_string(),
            cost: self.cost.clone(),
            instr: instr.to_vec(),
        }
    }
}

impl Target {
    pub fn from_serial(input: Def) -> Target {
        let mut patterns: Vec<instr::Pattern> = Vec::new();
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
