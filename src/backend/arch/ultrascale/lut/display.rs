use crate::backend::arch::ultrascale::lut::*;
use std::fmt;
use vast::v05::ast as Verilog;

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

impl fmt::Display for Lut {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Verilog::Stmt::from(self.clone()))
    }
}
