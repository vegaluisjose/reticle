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
        if instr.is_std() {
            let ty = instr.ty().clone();
            let op = instr.std_op().clone();
            let mut asm_instr = asm::InstrStd::new(op);
            asm_instr.set_dst_id(&instr.id());
            asm_instr.set_dst_ty(ty);
            for attr in instr.attrs().iter() {
                asm_instr.add_attr(attr.clone());
            }
            for param in instr.params().iter() {
                asm_instr.add_param(param.clone());
            }
            asm::Instr::from(asm_instr)
        } else {
            panic!("only std instr conversion supported")
        }
    }
}

impl From<asm::Prog> for verilog::Module {
    fn from(prog: asm::Prog) -> Self {
        use crate::backend::arch::ultrascale::assembler::Assembler;
        let mut assembler = Assembler::default();
        assembler.emit(prog)
    }
}
