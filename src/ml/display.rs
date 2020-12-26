use crate::ml::ast::*;
use crate::util::pretty_print::PrettyPrint;
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

display!(Opt);
display!(OptValOp);
display!(OptVal);
display!(OpBasc);
display!(OpMach);
display!(BelLut);
display!(BelReg);
display!(BelCarry);
display!(Bel);
display!(Loc);
display!(InstrBasc);
display!(InstrMach);
display!(Instr);
display!(Prog);
