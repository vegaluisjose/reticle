use crate::lang::ast;
use crate::passes::select::block::*;
use crate::passes::select::dag::*;
use crate::passes::select::instr::*;
use std::convert::From;

// Once we support partitioning, then
// this function will likely change because
// inputs can come from either lut or dsp
fn create_instr_from_expr(expr: &ast::Expr) -> Instr {
    let op = Op::In;
    let ty = expr.ty().clone();
    let loc = Loc::Lut;
    Instr::new(op, ty, loc)
}

fn create_instr_from_instr(instr: &ast::Instr) -> Instr {
    let op = Op::from(instr.prim_op().clone());
    let ty = instr.ty().clone();
    let loc = Loc::from(instr.loc().clone());
    Instr::new(op, ty, loc)
}

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
