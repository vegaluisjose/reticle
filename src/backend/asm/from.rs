use crate::backend::asm::ast as asm;
use crate::backend::verilog;
use crate::lang::ast as lang;

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

// only for std instructions
impl From<lang::Instr> for asm::Instr {
    fn from(instr: lang::Instr) -> Self {
        asm::Instr::from(instr.std().clone())
    }
}

impl From<asm::Prog> for verilog::Module {
    fn from(prog: asm::Prog) -> Self {
        use crate::backend::arch::ultrascale::assembler::Assembler;
        let mut assembler = Assembler::default();
        assembler.emit(prog)
    }
}
