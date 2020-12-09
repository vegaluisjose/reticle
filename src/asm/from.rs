use crate::asm::ast::*;

impl From<InstrWire> for Instr {
    fn from(instr: InstrWire) -> Self {
        Instr::Wire(instr)
    }
}

impl From<InstrAsm> for Instr {
    fn from(instr: InstrAsm) -> Self {
        Instr::Asm(instr)
    }
}
