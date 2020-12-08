use crate::v2::ir::ast::*;

impl From<ExprTup> for Expr {
    fn from(tup: ExprTup) -> Self {
        Expr::Tup(tup)
    }
}

impl From<InstrCall> for Instr {
    fn from(instr: InstrCall) -> Self {
        Instr::Call(instr)
    }
}

impl From<InstrWire> for Instr {
    fn from(instr: InstrWire) -> Self {
        Instr::Wire(instr)
    }
}

impl From<InstrComp> for Instr {
    fn from(instr: InstrComp) -> Self {
        Instr::Comp(instr)
    }
}