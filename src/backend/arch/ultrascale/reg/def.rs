#[derive(Clone, Debug)]
pub enum Ty {
    Fdre,
    Fdse,
}

#[derive(Clone, Debug)]
pub struct Slice {
    pub x: u32,
    pub y: u32,
}

#[derive(Clone, Debug)]
pub struct Bel {
    pub letter: String,
    pub number: u32,
}

#[derive(Clone, Debug)]
pub struct Loc {
    pub slice: Slice,
    pub bel: Bel,
}

#[derive(Clone, Debug)]
pub enum Expr {
    Ref(String),
    Index(String, u32),
}

#[derive(Clone, Debug)]
pub struct Reg {
    pub id: String,
    pub ty: Ty,
    pub clock: Expr,
    pub reset: Expr,
    pub en: Expr,
    pub input: Expr,
    pub output: Expr,
    pub loc: Option<Loc>,
}
