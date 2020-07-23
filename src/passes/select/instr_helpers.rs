use crate::lang::ast;
use crate::passes::select::instr::*;

// Once we support partitioning, then
// this function will likely change because
// inputs can come from either lut or dsp
pub fn create_instr_from_expr(expr: &ast::Expr) -> Instr {
    let op = Op::In;
    let ty = expr.ty().clone();
    let loc = Loc::Lut;
    Instr::new(op, ty, loc)
}

pub fn create_instr_from_instr(instr: &ast::Instr) -> Instr {
    let op = Op::from(instr.prim_op().clone());
    let ty = instr.ty().clone();
    let loc = Loc::from(instr.loc().clone());
    Instr::new(op, ty, loc)
}

impl Instr {
    pub fn new(op: Op, ty: Ty, loc: Loc) -> Instr {
        Instr {
            op: op,
            ty: ty,
            loc: loc,
        }
    }
}

impl Pattern {
    pub fn new_with_cost(name: &str, cost: u32) -> Pattern {
        Pattern {
            name: name.to_string(),
            cost: cost.clone(),
            instr: Vec::new(),
        }
    }

    pub fn add_instr(&mut self, instr: Instr) {
        self.instr.push(instr);
    }
}
