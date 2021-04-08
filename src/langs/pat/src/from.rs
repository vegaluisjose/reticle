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

impl From<Pat> for InstrMap {
    fn from(input: Pat) -> InstrMap {
        let mut map = InstrMap::new();
        for instr in input.body() {
            if let Some(term) = instr.dst().term() {
                if let Some(id) = term.id() {
                    let value = instr.clone();
                    map.insert(id, value);
                }
            }
        }
        map
    }
}
