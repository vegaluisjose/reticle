use crate::backend::arch::ultrascale::reg::*;
use crate::backend::verilog;
use std::fmt;

impl fmt::Display for Ty {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Ty::Fdre => "FDRE",
            Ty::Fdse => "FDSE",
        };
        write!(f, "{}", name)
    }
}

impl fmt::Display for RegPrim {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", verilog::Parallel::from(self.clone()))
    }
}
