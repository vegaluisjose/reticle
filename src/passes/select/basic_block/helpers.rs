use crate::lang::ast;
use crate::passes::select::basic_block::*;

impl BasicBlock {

    pub fn add_instr(&mut self, instr: &ast::Instr) {
        self.body.push(instr.clone());
    }

    pub fn body(&self) -> &Vec<ast::Instr> {
        &self.body
    }
}
