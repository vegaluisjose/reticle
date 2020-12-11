use crate::ir::ast::*;
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

display!(Ty);
display!(Prim);
display!(ExprTup);
display!(Expr);
display!(OpWire);
display!(CompOp);
display!(CallOp);
display!(InstrCall);
display!(InstrComp);
display!(InstrWire);
display!(Instr);
display!(Sig);
display!(Def);
display!(Prog);
