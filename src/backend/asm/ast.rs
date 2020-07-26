use crate::lang::ast;

// reuse from reticle ast for the time being
pub type Id = ast::Id;
pub type Ty = ast::Ty;
pub type LocTy = ast::Loc;
pub type Port = ast::Port;
pub type Sig = ast::Sig;

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
pub enum Expr {
    Ref(Id, Ty),
}

#[derive(Clone, Debug)]
pub struct Instr {
    pub ty: Ty,
    pub op: Id,
    pub loc: Loc,
    pub area: u32,
    pub dst: Option<Id>,
    pub params: Vec<Expr>,
}

#[derive(Clone, Debug)]
pub struct Prog {
    pub sig: Sig,
    pub body: Vec<Instr>,
    pub target: String,
}
