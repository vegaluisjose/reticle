use crate::lang::ast;

pub type Id = ast::Id;
pub type Ty = ast::Ty;
pub type LocTy = ast::Loc;

#[derive(Clone, Debug)]
pub enum CoorTy {
    Var,
    Lit(u32),
}

#[derive(Clone, Debug)]
pub struct Loc {
    ty: LocTy,
    x: CoorTy,
    y: CoorTy,
}

#[derive(Clone, Debug)]
pub struct Instr {
    pub id: Id,
    pub ty: Ty,
    pub op: Id,
    pub loc: Loc,
    pub params: Vec<Id>,
}

#[derive(Clone, Debug)]
pub enum Port {
    Input { id: Id, ty: Ty },
    Output { id: Id, ty: Ty },
}

#[derive(Clone, Debug)]
pub struct Sig {
    pub id: Id,
    pub inputs: Vec<Port>,
    pub outputs: Vec<Port>,
}

#[derive(Clone, Debug)]
pub struct Prog {
    pub sig: Sig,
    pub body: Vec<Instr>,
}