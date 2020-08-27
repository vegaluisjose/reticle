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
}
