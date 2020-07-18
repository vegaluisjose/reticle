use std::convert::From;
use crate::lang::ast;
use crate::passes::select::dag::*;

impl From<ast::Prog> for SDag {
    fn from(program: ast::Prog) -> Self {
        SDag::new()
    }
}