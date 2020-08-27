use crate::backend::arch::ultrascale::prim::*;

impl Prim {
    pub fn is_reg(&self) -> bool {
        match self {
            Prim::Reg(_) => true,
            _ => false,
        }
    }
    pub fn is_lut(&self) -> bool {
        match self {
            Prim::Lut(_) => true,
            _ => false,
        }
    }
    pub fn set_id(&mut self, id: &str) {
        match self {
            Prim::Reg(reg) => reg.set_id(id),
            Prim::Lut(lut) => lut.set_id(id),
        }
    }
    pub fn set_clock(&mut self, clock: &str) {
        match self {
            Prim::Reg(reg) => reg.set_clock(clock),
            _ => panic!("Error: prim does not support clock"),
        }
    }
    pub fn set_reset(&mut self, reset: &str) {
        match self {
            Prim::Reg(reg) => reg.set_reset(reset),
            _ => panic!("Error: prim does not support reset"),
        }
    }
}
