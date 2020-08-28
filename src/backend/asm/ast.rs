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
pub enum Instr {
    Std {
        op: StdOp,
        dst: Expr,
        attrs: Vec<Expr>,
        params: Vec<Expr>,
    },
    Prim {
        id: Id,
        ty: Ty,
        op: Id,
        attrs: Vec<Expr>,
        params: Vec<Expr>,
        loc: Loc,
    },
}

#[derive(Clone, Debug)]
pub struct Prog {
    pub sig: Sig,
    pub body: Vec<Instr>,
}
