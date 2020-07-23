use crate::passes::select::basic_block::*;
use crate::passes::select::instr::helpers::*;
use crate::passes::select::sdag::*;
use std::convert::From;

impl From<BasicBlock> for SDag {
    fn from(block: BasicBlock) -> Self {
        let mut sdag = SDag::default();
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
