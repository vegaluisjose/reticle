use crate::lang::ast::*;
use crate::lang::interp::state::State;
use crate::lang::interp::ty::Value;

pub trait Eval {
    fn is_ready(&self, state: &State) -> bool;
    fn eval(&self, state: &State) -> Value;
}

impl Eval for Instr {
    fn is_ready(&self, state: &State) -> bool {
        match self {
            Instr::Std {
                id: _,
                ty: _,
                op: StdOp::Identity,
                attrs: _,
                params,
            } => state.contains(&params[0].id()),
            _ => unimplemented!("Prim instr not supported yet"),
        }
    }

    fn eval(&self, state: &State) -> Value {
        match self {
            Instr::Std {
                id: _,
                ty: _,
                op: StdOp::Identity,
                attrs: _,
                params,
            } => state.get(&params[0].id()),
            _ => unimplemented!("Prim instr not supported yet"),
        }
    }
}
