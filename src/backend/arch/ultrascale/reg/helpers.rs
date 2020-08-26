use crate::backend::arch::ultrascale::reg::*;

impl Reg {
    pub fn new_fdre(id: &str) -> Reg {
        Reg {
            id: id.to_string(),
            ty: Ty::Fdre,
            clock: String::new(),
            reset: String::new(),
            en: String::new(),
            input: String::new(),
            output: String::new(),
            loc: None,
        }
    }
    pub fn new_fdse(id: &str) -> Reg {
        Reg {
            id: id.to_string(),
            ty: Ty::Fdse,
            clock: String::new(),
            reset: String::new(),
            en: String::new(),
            input: String::new(),
            output: String::new(),
            loc: None,
        }
    }
    pub fn is_fdre(&self) -> bool {
        match self.ty {
            Ty::Fdre => true,
            _ => false,
        }
    }
    pub fn is_fdse(&self) -> bool {
        match self.ty {
            Ty::Fdse => true,
            _ => false,
        }
    }
    pub fn id(&self) -> String {
        self.id.to_string()
    }
    pub fn ty(&self) -> &Ty {
        &self.ty
    }
    pub fn clock(&self) -> String {
        self.clock.to_string()
    }
    pub fn reset(&self) -> String {
        self.reset.to_string()
    }
    pub fn en(&self) -> String {
        self.en.to_string()
    }
    pub fn input(&self) -> String {
        self.input.to_string()
    }
    pub fn output(&self) -> String {
        self.output.to_string()
    }
    pub fn set_clock(&mut self, clock: &str) {
        self.clock = clock.to_string();
    }
    pub fn set_reset(&mut self, reset: &str) {
        self.reset = reset.to_string();
    }
    pub fn set_en(&mut self, en: &str) {
        self.en = en.to_string();
    }
    pub fn set_input(&mut self, input: &str) {
        self.input = input.to_string();
    }
    pub fn set_output(&mut self, output: &str) {
        self.output = output.to_string();
    }
    pub fn set_loc(&mut self, loc: Loc) {
        self.loc = Some(loc);
    }
}
