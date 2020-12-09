use crate::tdl::ast::*;

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
