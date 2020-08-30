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
pub struct Reg {
    pub id: String,
    pub ty: Ty,
    pub clock: String,
    pub reset: String,
    pub en: String,
    pub input: String,
    pub output: String,
    pub loc: Option<Loc>,
}
