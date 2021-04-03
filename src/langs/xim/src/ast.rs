use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use xir::ast as xir;

pub type Id = xir::Id;
pub type Ty = xir::Ty;
pub type Prim = xir::Prim;
pub type OpCoord = xir::OpCoord;
pub type OpBasc = xir::OpBasc;
pub type OpMach = xir::OpMach;
pub type ExprTerm = xir::ExprTerm;
pub type ExprTup = xir::ExprTup;
pub type Expr = xir::Expr;
pub type ExprCoord = xir::ExprCoord;
pub type BelLut = xir::BelLut;
pub type BelReg = xir::BelReg;
pub type BelCarry = xir::BelCarry;
pub type BelDsp = xir::BelDsp;
pub type Bel = xir::Bel;
pub type Loc = xir::Loc;
pub type InstrMach = xir::InstrMach;
pub type InstrBasc = xir::InstrBasc;
pub type Instr = xir::Instr;

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, Eq, Hash)]
pub struct Sig {
    pub id: Id,
    pub input: Expr,
    pub output: Expr,
    pub area: u64,
    pub perf: u64,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, Eq, Hash, Default)]
pub struct Imp {
    pub sig: Sig,
    pub body: Vec<Instr>,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, Eq, Default)]
pub struct Target {
    pub imp: HashMap<Id, Imp>,
}
