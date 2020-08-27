use crate::backend::arch::ultrascale::lut::*;

impl LutPrim {
    pub fn new_lut2() -> LutPrim {
        LutPrim {
            id: String::new(),
            init: "0".to_string(),
            ty: Ty::Lut2,
            inputs: Vec::new(),
            output: String::new(),
            loc: None,
        }
    }

    pub fn new_lut3() -> LutPrim {
        LutPrim {
            id: String::new(),
            init: "0".to_string(),
            ty: Ty::Lut3,
            inputs: Vec::new(),
            output: String::new(),
            loc: None,
        }
    }

    pub fn new_lut4() -> LutPrim {
        LutPrim {
            id: String::new(),
            init: "0".to_string(),
            ty: Ty::Lut4,
            inputs: Vec::new(),
            output: String::new(),
            loc: None,
        }
    }

    pub fn new_lut5() -> LutPrim {
        LutPrim {
            id: String::new(),
            init: "0".to_string(),
            ty: Ty::Lut5,
            inputs: Vec::new(),
            output: String::new(),
            loc: None,
        }
    }

    pub fn new_lut6() -> LutPrim {
        LutPrim {
            id: String::new(),
            init: "0".to_string(),
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

    pub fn init(&self) -> String {
        self.init.to_string()
    }

    pub fn inputs(&self) -> &Vec<String> {
        &self.inputs
    }

    pub fn output(&self) -> String {
        self.output.to_string()
    }

    pub fn set_id(&mut self, id: &str) {
        self.id = id.to_string();
    }

    pub fn set_init(&mut self, value: &str) {
        self.init = value.to_string();
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
