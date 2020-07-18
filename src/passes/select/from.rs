use std::convert::From;
use crate::lang::ast;
use crate::passes::select::dag::*;
use crate::passes::select::instr::*;

fn create_input_from_expr(expr: &ast::Expr) -> Instr {
    let op = Op::Inp;
    let ty = expr.ty().clone();
    let loc = Loc::Var;
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
