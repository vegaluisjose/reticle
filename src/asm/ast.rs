use crate::ir::ast as ir;

pub type Id = ir::Id;
pub type Ty = ir::Ty;
pub type Prim = ir::Prim;
pub type ExprTup = ir::ExprTup;
pub type Expr = ir::Expr;
pub type WireOp = ir::WireOp;
pub type InstrWire = ir::InstrWire;
pub type Sig = ir::Sig;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ExprCoord {
    Any,
    Var(Id),
    Val(u64),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Loc {
    pub prim: Prim,
    pub x: ExprCoord,
    pub y: ExprCoord,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum AsmOp {
    Op(Id),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct InstrAsm {
    pub op: AsmOp,
    pub dst: Expr,
    pub attr: Expr,
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
