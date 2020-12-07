use crate::v2::il::ast as il;

pub type Id = il::Id;
pub type Ty = il::Ty;
pub type Prim = il::Prim;
pub type ExprTup = il::ExprTup;
pub type Expr = il::Expr;
pub type WireOp = il::WireOp;
pub type InstrWire = il::InstrWire;
pub type Sig = il::Sig;

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
