use crate::util::pretty_print::PrettyPrint;
use crate::v2::il::ast::*;
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
display!(ExprTup);
display!(Expr);
display!(WireOp);
display!(CompOp);
display!(CallOp);
display!(InstrCall);
display!(InstrComp);
display!(InstrWire);
display!(Instr);
display!(Sig);
display!(Def);
display!(Prog);
