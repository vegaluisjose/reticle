use std::fmt;
use vast::v05::ast::Instance;

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

impl Lut {
    pub fn new_lut2(id: &str) -> Lut {
        Lut {
            id: id.to_string(),
            init: 0,
            ty: Ty::Lut2,
            inputs: Vec::new(),
            output: String::new(),
            loc: None,
        }
    }
    pub fn id(&self) -> String {
        self.id.to_string()
    }
    pub fn ty(&self) -> &Ty {
        &self.ty
    }
}

impl fmt::Display for Ty {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Ty::Lut2 => "LUT2",
            Ty::Lut3 => "LUT3",
            Ty::Lut4 => "LUT4",
            Ty::Lut5 => "LUT5",
            Ty::Lut6 => "LUT6",
        };
        write!(f, "{}", name)
    }
}

impl fmt::Display for Lut {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Instance::from(self.clone()))
    }
}
