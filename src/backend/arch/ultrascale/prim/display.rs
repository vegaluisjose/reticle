use crate::backend::arch::ultrascale::prim::ast::*;
use crate::backend::verilog;
use std::fmt;

impl fmt::Display for LutTy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            LutTy::Lut2 => "LUT2",
            LutTy::Lut3 => "LUT3",
            LutTy::Lut4 => "LUT4",
            LutTy::Lut5 => "LUT5",
            LutTy::Lut6 => "LUT6",
        };
        write!(f, "{}", name)
    }
}

impl fmt::Display for Lut {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", verilog::Stmt::from(self.clone()))
    }
}

impl fmt::Display for RegTy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            RegTy::Fdre => "FDRE",
            RegTy::Fdse => "FDSE",
        };
        write!(f, "{}", name)
    }
}

impl fmt::Display for Reg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", verilog::Stmt::from(self.clone()))
    }
}

impl fmt::Display for Dsp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", verilog::Stmt::from(self.clone()))
    }
}
