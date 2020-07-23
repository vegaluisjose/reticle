use crate::lang::ast;
use crate::passes::select::basic_block::*;
use std::convert::From;

// for now, a block consist of all the instr
// in the body and there is no partitioning
// yet. Likely, this is going to change very
// soon
impl From<ast::Def> for BasicBlock {
    fn from(def: ast::Def) -> Self {
        let mut block = BasicBlock::new();
        for instr in def.body().iter() {
            block.add_instr(instr);
        }
        block
    }
}
