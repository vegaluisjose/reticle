use crate::ir::ast as ir;
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

impl From<PatInstr> for ir::Instr {
    fn from(instr: PatInstr) -> Self {
        match instr {
            PatInstr::Wire(instr) => ir::Instr::Wire(instr),
            PatInstr::Comp(instr) => ir::Instr::Comp(instr),
        }
    }
}

impl From<Pat> for ir::InstrMap {
    fn from(input: Pat) -> Self {
        let mut imap = ir::InstrMap::new();
        for instr in input.body() {
            if let Some(term) = instr.dst().term() {
                if let Some(id) = term.id() {
                    let ir_instr = ir::Instr::from(instr.clone());
                    imap.insert(id, ir_instr.clone());
                }
            }
        }
        imap
    }
}
