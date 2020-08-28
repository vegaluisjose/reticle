use crate::backend::arch::ultrascale::lut::*;

impl Expr {
    pub fn new_ref(name: &str) -> Expr {
        Expr::Ref(name.to_string())
    }
    pub fn new_index(name: &str, index: u32) -> Expr {
        Expr::Index(name.to_string(), index)
    }
}

impl LutPrim {
    pub fn new_lut2() -> LutPrim {
        LutPrim {
            id: String::new(),
            init: "0".to_string(),
            ty: Ty::Lut2,
            inputs: Vec::new(),
            output: Expr::default(),
            loc: None,
        }
    }

    pub fn new_lut3() -> LutPrim {
        LutPrim {
            id: String::new(),
            init: "0".to_string(),
            ty: Ty::Lut3,
            inputs: Vec::new(),
            output: Expr::default(),
            loc: None,
        }
    }

    pub fn new_lut4() -> LutPrim {
        LutPrim {
            id: String::new(),
            init: "0".to_string(),
            ty: Ty::Lut4,
            inputs: Vec::new(),
            output: Expr::default(),
            loc: None,
        }
    }

    pub fn new_lut5() -> LutPrim {
        LutPrim {
            id: String::new(),
            init: "0".to_string(),
            ty: Ty::Lut5,
            inputs: Vec::new(),
            output: Expr::default(),
            loc: None,
        }
    }

    pub fn new_lut6() -> LutPrim {
        LutPrim {
            id: String::new(),
            init: "0".to_string(),
            ty: Ty::Lut6,
            inputs: Vec::new(),
            output: Expr::default(),
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

    pub fn inputs(&self) -> &Vec<Expr> {
        &self.inputs
    }

    pub fn output(&self) -> &Expr {
        &self.output
    }

    pub fn set_id(&mut self, id: &str) {
        self.id = id.to_string();
    }

    pub fn set_init(&mut self, value: &str) {
        self.init = value.to_string();
    }

    pub fn add_input(&mut self, name: &str) {
        self.inputs.push(Expr::new_ref(name));
    }

    pub fn add_input_with_index(&mut self, name: &str, index: u32) {
        self.inputs.push(Expr::new_index(name, index));
    }

    pub fn set_output(&mut self, name: &str) {
        self.output = Expr::new_ref(name);
    }

    pub fn set_output_with_index(&mut self, name: &str, index: u32) {
        self.output = Expr::new_index(name, index);
    }

    pub fn set_loc(&mut self, loc: Loc) {
        self.loc = Some(loc);
    }
}
