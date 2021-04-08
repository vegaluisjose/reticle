use crate::ast::*;
use ir::ast as ir;

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

impl From<Instr> for ir::Instr {
    fn from(instr: Instr) -> Self {
        match instr {
            Instr::Wire(instr) => ir::Instr::from(instr),
            Instr::Prim(instr) => ir::Instr::from(instr),
        }
    }
}

impl From<Pat> for ir::InstrMap {
    fn from(input: Pat) -> ir::InstrMap {
        let mut map = ir::InstrMap::new();
        for instr in input.body() {
            if let Some(term) = instr.dst().term() {
                if let Some(id) = term.id() {
                    let value = ir::Instr::from(instr.clone());
                    map.insert(id, value);
                }
            }
        }
        map
    }
}
