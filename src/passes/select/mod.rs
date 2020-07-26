use crate::backend::asm::ast as asm;
use crate::lang::ast as reticle;
use crate::passes::select::basic_block::BasicBlock;
use crate::passes::select::sdag::SDag;

pub mod basic_block;
pub mod instr;
pub mod sdag;

#[derive(Clone, Debug)]
pub struct Select {
    source: reticle::Prog,
    target: String,
}

impl Select {
    pub fn new(prog: reticle::Prog, target: &str) -> Select {
        Select {
            source: prog,
            target: target.to_string(),
        }
    }

    pub fn target(&self) -> String {
        self.target.to_string()
    }

    pub fn run(&self) -> asm::Prog {
        let block = BasicBlock::from(self.source.defs[0].clone());
        let mut sdag = SDag::new(block, &self.target());
        sdag.select("y"); // fix this
        let instr = sdag.codegen("y"); // fix this
        let mut asm = asm::Prog::new(self.source.clone(), &self.target());
        for instr in instr.iter() {
            asm.add_instr(instr.clone());
        }
        asm
    }
}
