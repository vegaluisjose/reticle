use crate::util::pretty_print::PrettyPrint;
use crate::v2::asm::ast::*;
use std::fmt;

macro_rules! display {
    ($ty:tt) => {
        impl fmt::Display for $ty {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}", self.to_pretty())
            }
        }
    };
}

display!(ExprCoord);
display!(Loc);
display!(AsmOp);
display!(InstrAsm);
display!(Instr);
display!(Prog);
