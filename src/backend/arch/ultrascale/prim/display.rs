use crate::backend::arch::ultrascale::prim::Prim;
use std::fmt;

impl fmt::Display for Prim {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string = match self {
            Prim::Reg(reg) => reg.to_string(),
            Prim::Lut(lut) => lut.to_string(),
        };
        write!(f, "{}", string)
    }
}
