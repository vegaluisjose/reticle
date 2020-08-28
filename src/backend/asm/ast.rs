use crate::lang::ast;

// The main difference between Reticle and Reticle-asm are:
// 1. Primitive instructions support location expressions
// 2. Placement can be achieved by solving these location expressions

pub type Id = ast::Id;
pub type Ty = ast::Ty;
pub type LocTy = ast::Loc;
pub type Port = ast::Port;
pub type Sig = ast::Sig;
pub type Expr = ast::Expr;
pub type StdOp = ast::StdOp;

#[derive(Clone, Debug)]
pub enum LocExpr {
    Hole,
    Var(Id),
    Lit(u32),
}

#[derive(Clone, Debug)]
pub struct Loc {
    pub ty: LocTy,
    pub x: LocExpr,
    pub y: LocExpr,
}

#[derive(Clone, Debug)]
pub struct InstrStd {
    pub op: StdOp,
    pub dst: Expr,
    pub attrs: Vec<Expr>,
    pub params: Vec<Expr>,
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
    pub sig: Sig,
    pub body: Vec<Instr>,
}
