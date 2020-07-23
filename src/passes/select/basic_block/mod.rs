use crate::lang::ast;

pub mod from;
pub mod helpers;

#[derive(Default)]
pub struct BasicBlock {
    pub body: Vec<ast::Instr>,
}
