use crate::lang::ast;
use crate::passes::select::instr::*;
use std::convert::From;

impl From<ast::PrimOp> for Op {
    fn from(prim_op: ast::PrimOp) -> Self {
        match prim_op {
            ast::PrimOp::Reg => Op::Reg,
            ast::PrimOp::Add => Op::Add,
            ast::PrimOp::Sub => Op::Sub,
            ast::PrimOp::Mul => Op::Mul,
            ast::PrimOp::Not => Op::Not,
            ast::PrimOp::And => Op::And,
            ast::PrimOp::Nand => Op::Nand,
            ast::PrimOp::Or => Op::Or,
            ast::PrimOp::Nor => Op::Nor,
            ast::PrimOp::Xor => Op::Xor,
            ast::PrimOp::Xnor => Op::Xnor,
            ast::PrimOp::Mux => Op::Mux,
            ast::PrimOp::Equal => Op::Equal,
            ast::PrimOp::Nequal => Op::Nequal,
            ast::PrimOp::Gt => Op::Gt,
            ast::PrimOp::Lt => Op::Lt,
            ast::PrimOp::Ge => Op::Ge,
            ast::PrimOp::Le => Op::Le,
        }
    }
}

impl From<ast::Loc> for Loc {
    fn from(loc: ast::Loc) -> Self {
        match loc {
            ast::Loc::Hole => Loc::Hole,
            ast::Loc::Lut => Loc::Lut,
            ast::Loc::Dsp => Loc::Dsp,
            ast::Loc::Lum => Loc::Lum,
            ast::Loc::Ram => Loc::Ram,
        }
    }
}