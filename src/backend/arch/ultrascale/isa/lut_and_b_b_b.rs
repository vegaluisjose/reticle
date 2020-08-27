use crate::backend::arch::ultrascale::lut::LutPrim;
use crate::backend::arch::ultrascale::prim::{Block, Prim, ToBlock};
use crate::backend::asm::ast as asm;

#[derive(Clone, Debug)]
pub struct LutAndBBB {
    pub instr: asm::Instr,
}

impl LutAndBBB {
    pub fn new(instr: asm::Instr) -> LutAndBBB {
        LutAndBBB { instr }
    }
}

impl ToBlock for LutAndBBB {
    fn to_block(&self) -> Block {
        let mut block = Block::new();
        let mut lut = LutPrim::new_lut2();
        lut.add_input(&self.instr.params()[0].id());
        lut.add_input(&self.instr.params()[1].id());
        lut.set_output(&self.instr.id());
        lut.set_init(8);
        block.push(Prim::from(lut));
        block
    }
}
