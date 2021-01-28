use crate::asm::ast as asm;
use crate::ir::ast as ir;
use crate::xl::ast as xl;
use std::collections::HashMap;

pub type Id = ir::Id;
pub type Ty = ir::Ty;
pub type Prim = ir::Prim;
pub type ExprTerm = ir::ExprTerm;
pub type ExprTup = ir::ExprTup;
pub type Expr = ir::Expr;
pub type ExprCoord = asm::ExprCoord;
pub type OpWire = ir::OpWire;
pub type OpComp = ir::OpComp;
pub type InstrWire = ir::InstrWire;
pub type InstrComp = ir::InstrComp;
pub type InstrXL = xl::Instr;

#[derive(Clone, Debug)]
pub enum PatInstr {
    Wire(InstrWire),
    Comp(InstrComp),
}

#[derive(Clone, Debug)]
pub enum ImpInstr {
    XL(InstrXL),
}

#[derive(Clone, Debug)]
pub struct PatSig {
    pub id: Id,
    pub prim: Prim,
    pub area: u64,
    pub lat: u64,
    pub input: Expr,
    pub output: Expr,
}

#[derive(Clone, Debug)]
pub struct ImpSig {
    pub id: Id,
    pub x: ExprCoord,
    pub y: ExprCoord,
    pub input: Expr,
    pub output: Expr,
}

#[derive(Clone, Debug)]
pub struct Pat {
    pub sig: PatSig,
    pub body: Vec<PatInstr>,
}

#[derive(Clone, Debug)]
pub struct Imp {
    pub sig: ImpSig,
    pub body: Vec<ImpInstr>,
}

#[derive(Clone, Debug)]
pub enum Des {
    Pat(Pat),
    Imp(Imp),
}

#[derive(Clone, Debug, Default)]
pub struct Target {
    pub pat: HashMap<Id, Pat>,
    pub imp: HashMap<Id, Imp>,
}
