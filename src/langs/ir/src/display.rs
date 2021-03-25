use crate::ast::*;
use utils::pretty_print::PrettyPrint;
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

display!(Ty);
display!(Prim);
display!(ExprTerm);
display!(ExprTup);
display!(Expr);
display!(OpWire);
display!(OpPrim);
display!(OpCall);
display!(InstrCall);
display!(InstrPrim);
display!(InstrWire);
display!(Instr);
display!(Sig);
display!(Def);
display!(Prog);
