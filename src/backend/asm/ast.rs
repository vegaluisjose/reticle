use crate::passes::select::instr as sel;

pub type Id = sel::Id;
pub type Ty = sel::Ty;
pub type LocTy = sel::Loc;

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

impl Expr {
    pub fn new_ref(name: &str, ty: Ty) -> Expr {
        Expr::Ref(name.to_string(), ty)
    }
}

impl Instr {
    pub fn set_dst(&mut self, name: &str) {
        self.dst = Some(name.to_string());
    }

    pub fn add_param(&mut self, expr: Expr) {
        self.params.push(expr);
    }
}