use crate::asm::ast as asm;
use crate::lang::ast as lang;
use crate::passes::select::asmgen;

impl From<asm::InstrStd> for asm::Instr {
    fn from(std: asm::InstrStd) -> Self {
        asm::Instr::Std(std)
    }
}

impl From<asm::InstrPhy> for asm::Instr {
    fn from(prim: asm::InstrPhy) -> Self {
        asm::Instr::Prim(prim)
    }
}

impl From<lang::Prog> for asm::Prog {
    fn from(prog: lang::Prog) -> Self {
        asmgen(prog)
    }
}

impl From<lang::Instr> for asm::Instr {
    fn from(instr: lang::Instr) -> Self {
        asm::Instr::from(instr.std().clone())
    }
}
