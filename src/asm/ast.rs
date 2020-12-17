use crate::ir::ast as ir;
use std::rc::Rc;

pub type Id = ir::Id;
pub type Ty = ir::Ty;
pub type Prim = ir::Prim;
pub type ExprTup = ir::ExprTup;
pub type Expr = ir::Expr;
pub type OpWire = ir::OpWire;
pub type InstrWire = ir::InstrWire;
pub type Sig = ir::Sig;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum OpCoord {
    Add,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ExprCoord {
    Any,
    Var(Id),
    Val(u64),
    Bin(OpCoord, Rc<ExprCoord>, Rc<ExprCoord>),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Loc {
    pub prim: Prim,
    pub x: ExprCoord,
    pub y: ExprCoord,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum OpAsm {
    Op(Id),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct InstrAsm {
    pub op: OpAsm,
    pub dst: Expr,
    pub arg: Expr,
    pub loc: Loc,
    pub area: u64,
    pub lat: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Instr {
    Wire(InstrWire),
    Asm(InstrAsm),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Prog {
    pub sig: Sig,
    pub body: Vec<Instr>,
}
