use crate::backend::asm::ast as asm;
use crate::lang::ast as lang;

// only for std instructions
impl From<lang::Instr> for asm::Instr {
    fn from(instr: lang::Instr) -> Self {
        if instr.is_std() {
            let ty = instr.ty().clone();
            let op = instr.std_op().clone();
            let mut asm_instr = asm::Instr::new_std(&instr.id(), ty, op);
            for attr in instr.attrs().iter() {
                asm_instr.add_attr(attr.clone());
            }
            for param in instr.params().iter() {
                asm_instr.add_param(param.clone());
            }
            asm_instr
        } else {
            panic!("only std instr conversion supported")
        }
    }
}