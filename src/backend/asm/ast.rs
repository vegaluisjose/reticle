use crate::lang::ast;

// The main difference between Reticle and Reticle-asm are:
// 1. Reticle-asm replace prim-instructions with asm-instructions
// 2. Asm-instructions support location expressions
// 3. Because of this, asm-instructions are used for doing placement

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
        id: Id,
        ty: Ty,
        op: StdOp,
        attrs: Vec<Expr>,
        params: Vec<Expr>,
    },
    Asm {
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
