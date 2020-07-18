use std::convert::From;
use crate::lang::ast;
use crate::passes::select::dag::*;

impl From<ast::Def> for SDag {
    fn from(def: ast::Def) -> Self {
        let sdag = SDag::new();
        for instr in def.body.iter() {
            println!("{}", instr);
        }
        sdag
    }
}

impl From<ast::Prog> for SDag {
    fn from(prog: ast::Prog) -> Self {
        assert!(prog.defs.len() == 1, "Error: only single definition allowed atm");
        SDag::from(prog.defs[0].clone())
    }
}