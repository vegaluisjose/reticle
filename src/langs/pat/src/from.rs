use crate::ast::*;

impl From<InstrWire> for Instr {
    fn from(instr: InstrWire) -> Self {
        Instr::Wire(instr)
    }
}

impl From<InstrPrim> for Instr {
    fn from(instr: InstrPrim) -> Self {
        Instr::Prim(instr)
    }
}
