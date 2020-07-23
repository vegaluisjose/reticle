use crate::passes::select::block::*;
use crate::lang::ast;

impl BasicBlock {
    pub fn new() -> BasicBlock {
        BasicBlock { body: Vec::new() }
    }

    pub fn add_instr(&mut self, instr: &ast::Instr) {
        self.body.push(instr.clone());
    }

    pub fn body(&self) -> &Vec<ast::Instr> {
        &self.body
    }
}
