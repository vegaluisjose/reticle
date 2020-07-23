use crate::lang::ast;

pub mod from;
pub mod helpers;

pub struct BasicBlock {
    pub body: Vec<ast::Instr>,
}
