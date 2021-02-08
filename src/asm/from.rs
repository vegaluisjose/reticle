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

impl From<Id> for OpAsm {
    fn from(op: Id) -> Self {
        OpAsm::Op(op)
    }
}
