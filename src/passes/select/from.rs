use crate::lang::ast;
use crate::passes::select::basic_block::*;
use crate::passes::select::instr_helpers::*;
use crate::passes::select::sdag::*;
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

impl From<BasicBlock> for SDag {
    fn from(block: BasicBlock) -> Self {
        let mut sdag = SDag::new();
        for instr in block.body().iter() {
            match instr.params().len() {
                // binary op i.e. add
                2 => {
                    let params = instr.params();
                    let lhs = &params[0];
                    let rhs = &params[1];
                    if !sdag.contains_sdnode(&lhs.id()) {
                        sdag.add_sdnode(&lhs.id(), create_instr_from_expr(&lhs));
                    }
                    if !sdag.contains_sdnode(&rhs.id()) {
                        sdag.add_sdnode(&rhs.id(), create_instr_from_expr(&rhs));
                    }
                    if !sdag.contains_sdnode(&instr.id()) {
                        sdag.add_sdnode(&instr.id(), create_instr_from_instr(instr));
                    }
                    sdag.add_sdedge(&instr.id(), &lhs.id());
                    sdag.add_sdedge(&instr.id(), &rhs.id());
                }
                _ => panic!("Error: only 2 params op supported atm"),
            }
        }
        sdag
    }
}
