use crate::lang::ast;
use crate::passes::select::block::*;
use crate::passes::select::dag::*;
use crate::passes::select::instr::*;
use std::convert::From;

fn create_instr_from_expr(expr: &ast::Expr) -> Instr {
    let op = Op::In;
    let ty = expr.ty().clone();
    let loc = Loc::Var;
    Instr::new(op, ty, loc)
}

fn create_instr_from_instr(instr: &ast::Instr) -> Instr {
    let op = Op::from(instr.placed_op().clone());
    let ty = instr.ty().clone();
    let loc = Loc::from(instr.loc().clone());
    Instr::new(op, ty, loc)
}

impl From<ast::PlacedOp> for Op {
    fn from(placed_op: ast::PlacedOp) -> Self {
        match placed_op {
            ast::PlacedOp::Reg => Op::Reg,
            ast::PlacedOp::Add => Op::Add,
            ast::PlacedOp::Sub => Op::Sub,
            ast::PlacedOp::Mul => Op::Mul,
            ast::PlacedOp::Not => Op::Not,
            ast::PlacedOp::And => Op::And,
            ast::PlacedOp::Nand => Op::Nand,
            ast::PlacedOp::Or => Op::Or,
            ast::PlacedOp::Nor => Op::Nor,
            ast::PlacedOp::Xor => Op::Xor,
            ast::PlacedOp::Xnor => Op::Xnor,
            ast::PlacedOp::Mux => Op::Mux,
            ast::PlacedOp::Equal => Op::Equal,
            ast::PlacedOp::Nequal => Op::Nequal,
            ast::PlacedOp::Gt => Op::Gt,
            ast::PlacedOp::Lt => Op::Lt,
            ast::PlacedOp::Ge => Op::Ge,
            ast::PlacedOp::Le => Op::Le,
        }
    }
}

impl From<ast::Loc> for Loc {
    fn from(loc: ast::Loc) -> Self {
        match loc {
            ast::Loc::Var => Loc::Var,
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
                2 => {
                    // binary op i.e. add
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
