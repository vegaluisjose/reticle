use crate::backend::arch::ultrascale::reg::*;

impl Reg {
    pub fn new_fdre() -> Reg {
        Reg {
            id: String::new(),
            ty: Ty::Fdre,
            clock: String::new(),
            reset: String::new(),
            en: String::new(),
            input: String::new(),
            output: String::new(),
            loc: None,
        }
    }
    pub fn new_fdse() -> Reg {
        Reg {
            id: String::new(),
            ty: Ty::Fdse,
            clock: String::new(),
            reset: String::new(),
            en: String::new(),
            input: String::new(),
            output: String::new(),
            loc: None,
        }
    }
    pub fn set_id(&mut self, id: &str) {
        self.id = id.to_string();
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