use crate::passes::select::instr::*;

impl Instr {
    pub fn new(op: Op, ty: Ty, loc: Loc) -> Instr {
        Instr {
            op: op,
            ty: ty,
            loc: loc,
        }
    }
}

impl Pattern {
    pub fn new_with_cost(name: &str, cost: u32) -> Pattern {
        Pattern {
            name: name.to_string(),
            cost: cost.clone(),
            instr: Vec::new(),
        }
    }

    pub fn add_instr(&mut self, instr: Instr) {
        self.instr.push(instr);
    }
}