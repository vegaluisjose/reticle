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
            Instr::Prim {
                id: _,
                ty: _,
                op: PrimOp::Reg,
                attrs: _,
                params,
                loc: _,
            } => state.contains(&params[0].id()) && state.contains(&params[1].id()),
            instr => unimplemented!("instr:{} not supported yet", instr),
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
            Instr::Prim {
                id,
                ty: _,
                op: PrimOp::Reg,
                attrs: _,
                params,
                loc: _,
            } => {
                let en = state.get(&params[1].id());
                if en > 0 {
                    state.get(&params[0].id())
                } else {
                    state.get(id)
                }
            }
            instr => unimplemented!("instr: {} not supported yet", instr),
        }
    }
}
