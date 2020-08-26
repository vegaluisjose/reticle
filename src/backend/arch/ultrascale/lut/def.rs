#[derive(Clone, Debug)]
pub enum Ty {
    Lut2,
    Lut3,
    Lut4,
    Lut5,
    Lut6,
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
pub struct Lut {
    pub id: String,
    pub init: u8,
    pub ty: Ty,
    pub inputs: Vec<String>,
    pub output: String,
    pub loc: Option<Loc>,
}
