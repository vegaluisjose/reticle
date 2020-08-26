use crate::backend::arch::ultrascale::reg::*;
use std::fmt;
use vast::v05::ast as Verilog;

impl fmt::Display for Ty {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Ty::Fdre => "FDRE",
            Ty::Fdse => "FDSE",
        };
        write!(f, "{}", name)
    }
}

impl fmt::Display for Reg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Verilog::Parallel::from(self.clone()))
    }
}
