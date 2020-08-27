use crate::backend::arch::ultrascale::lut::*;

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