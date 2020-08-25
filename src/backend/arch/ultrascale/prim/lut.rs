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

    pub fn new_lut3(id: &str) -> Lut {
        Lut {
            id: id.to_string(),
            init: 0,
            ty: Ty::Lut3,
            inputs: Vec::new(),
            output: String::new(),
            loc: None,
        }
    }

    pub fn new_lut4(id: &str) -> Lut {
        Lut {
            id: id.to_string(),
            init: 0,
            ty: Ty::Lut4,
            inputs: Vec::new(),
            output: String::new(),
            loc: None,
        }
    }

    pub fn new_lut5(id: &str) -> Lut {
        Lut {
            id: id.to_string(),
            init: 0,
            ty: Ty::Lut5,
            inputs: Vec::new(),
            output: String::new(),
            loc: None,
        }
    }

    pub fn new_lut6(id: &str) -> Lut {
        Lut {
            id: id.to_string(),
            init: 0,
            ty: Ty::Lut6,
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

    pub fn init(&self) -> u8 {
        self.init
    }

    pub fn inputs(&self) -> &Vec<String> {
        &self.inputs
    }

    pub fn output(&self) -> String {
        self.output.to_string()
    }

    pub fn set_init(&mut self, value: u8) {
        self.init = value;
    }

    pub fn add_input(&mut self, name: &str) {
        self.inputs.push(name.to_string());
    }

    pub fn set_output(&mut self, name: &str) {
        self.output = name.to_string();
    }

    pub fn set_loc(&mut self, loc: Loc) {
        self.loc = Some(loc);
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
