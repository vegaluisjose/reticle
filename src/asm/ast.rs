use crate::lang::ast;

// The main difference between Reticle and Reticle-asm are:
// 1. Primitive instructions support location expressions
// 2. Placement can be achieved by solving these location expressions

pub type Id = ast::Id;
pub type Ty = ast::Ty;
pub type LocTy = ast::Loc;
pub type Port = ast::Port;
pub type Signature = ast::Signature;
pub type Expr = ast::Expr;
pub type StdOp = ast::StdOp;
pub type InstrStd = ast::InstrStd;

#[derive(Clone, Debug)]
pub enum ExprCoord {
    Hole,
    Var(Id),
    Lit(u32),
}

#[derive(Clone, Debug)]
pub struct Loc {
    pub prim: LocTy,
    pub x: ExprCoord,
    pub y: ExprCoord,
}

#[derive(Clone, Debug)]
pub struct InstrPrim {
    pub op: Id,
    pub dst: Expr,
    pub attrs: Vec<Expr>,
    pub params: Vec<Expr>,
    pub loc: Loc,
}

#[derive(Clone, Debug)]
pub enum Instr {
    Std(InstrStd),
    Prim(InstrPrim),
}

#[derive(Clone, Debug)]
pub struct Prog {
    pub sig: Signature,
    pub body: Vec<Instr>,
}
