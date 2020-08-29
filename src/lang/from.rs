use crate::lang::ast::*;

impl From<InstrStd> for Instr {
    fn from(std: InstrStd) -> Self {
        Instr::Std(std)
    }
}

impl From<InstrPrim> for Instr {
    fn from(prim: InstrPrim) -> Self {
        Instr::Prim(prim)
    }
}
