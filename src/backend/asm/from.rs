use crate::backend::asm::ast as asm;
use crate::lang::ast as lang;
use crate::passes::select::asmgen;

impl From<asm::InstrStd> for asm::Instr {
    fn from(std: asm::InstrStd) -> Self {
        asm::Instr::Std(std)
    }
}

impl From<asm::InstrPrim> for asm::Instr {
    fn from(prim: asm::InstrPrim) -> Self {
        asm::Instr::Prim(prim)
    }
}

impl From<lang::Prog> for asm::Prog {
    fn from(prog: lang::Prog) -> Self {
        // always check for now
        asmgen(prog, true)
    }
}

impl From<lang::Instr> for asm::Instr {
    fn from(instr: lang::Instr) -> Self {
        asm::Instr::from(instr.std().clone())
    }
}
