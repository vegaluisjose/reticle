use crate::tdl::ast::*;

impl From<InstrWire> for PatInstr {
    fn from(instr: InstrWire) -> Self {
        PatInstr::Wire(instr)
    }
}

impl From<InstrComp> for PatInstr {
    fn from(instr: InstrComp) -> Self {
        PatInstr::Comp(instr)
    }
}

impl From<Imp> for Des {
    fn from(imp: Imp) -> Self {
        Des::Imp(imp)
    }
}

impl From<Pat> for Des {
    fn from(pat: Pat) -> Self {
        Des::Pat(pat)
    }
}
