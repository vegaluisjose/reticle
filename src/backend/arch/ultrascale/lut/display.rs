use crate::backend::arch::ultrascale::lut::*;
use crate::backend::verilog;
use std::fmt;

impl fmt::Display for Ty {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Ty::Lut2 => "LUT2",
            Ty::Lut3 => "LUT3",
            Ty::Lut4 => "LUT4",
            Ty::Lut5 => "LUT5",
            Ty::Lut6 => "LUT6",
        };
        write!(f, "{}", name)
    }
}

impl fmt::Display for LutPrim {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", verilog::Stmt::from(self.clone()))
    }
}
