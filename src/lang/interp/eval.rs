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
                op: PrimOp::Not,
                attrs: _,
                params,
                loc: _,
            } => state.contains(&params[0].id()),
            Instr::Prim {
                id: _,
                ty: _,
                op: PrimOp::Mux,
                attrs: _,
                params,
                loc: _,
            } => {
                state.contains(&params[0].id())
                    && state.contains(&params[1].id())
                    && state.contains(&params[2].id())
            }
            Instr::Prim {
                id: _,
                ty: _,
                op: _,
                attrs: _,
                params,
                loc: _,
            } => state.contains(&params[0].id()) && state.contains(&params[1].id()),
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
            Instr::Prim {
                id: _,
                ty: _,
                op: PrimOp::Add,
                attrs: _,
                params,
                loc: _,
            } => state.get(&params[0].id()) + state.get(&params[1].id()),
            Instr::Prim {
                id: _,
                ty: _,
                op: PrimOp::Sub,
                attrs: _,
                params,
                loc: _,
            } => state.get(&params[0].id()) - state.get(&params[1].id()),
            Instr::Prim {
                id: _,
                ty: _,
                op: PrimOp::Mul,
                attrs: _,
                params,
                loc: _,
            } => state.get(&params[0].id()) * state.get(&params[1].id()),
            Instr::Prim {
                id: _,
                ty: _,
                op: PrimOp::Not,
                attrs: _,
                params,
                loc: _,
            } => !state.get(&params[0].id()),
            Instr::Prim {
                id: _,
                ty: _,
                op: PrimOp::And,
                attrs: _,
                params,
                loc: _,
            } => state.get(&params[0].id()) & state.get(&params[1].id()),
            Instr::Prim {
                id: _,
                ty: _,
                op: PrimOp::Nand,
                attrs: _,
                params,
                loc: _,
            } => !(state.get(&params[0].id()) & state.get(&params[1].id())),
            Instr::Prim {
                id: _,
                ty: _,
                op: PrimOp::Or,
                attrs: _,
                params,
                loc: _,
            } => state.get(&params[0].id()) | state.get(&params[1].id()),
            Instr::Prim {
                id: _,
                ty: _,
                op: PrimOp::Nor,
                attrs: _,
                params,
                loc: _,
            } => !(state.get(&params[0].id()) | state.get(&params[1].id())),
            Instr::Prim {
                id: _,
                ty: _,
                op: PrimOp::Xor,
                attrs: _,
                params,
                loc: _,
            } => state.get(&params[0].id()) ^ state.get(&params[1].id()),
            Instr::Prim {
                id: _,
                ty: _,
                op: PrimOp::Xnor,
                attrs: _,
                params,
                loc: _,
            } => !(state.get(&params[0].id()) ^ state.get(&params[1].id())),
            Instr::Prim {
                id: _,
                ty: _,
                op: PrimOp::Mux,
                attrs: _,
                params,
                loc: _,
            } => {
                let en = state.get(&params[0].id());
                if en > 0 {
                    state.get(&params[1].id())
                } else {
                    state.get(&params[2].id())
                }
            }
            Instr::Prim {
                id: _,
                ty: _,
                op: PrimOp::Equal,
                attrs: _,
                params,
                loc: _,
            } => (state.get(&params[0].id()) == state.get(&params[1].id())) as i64,
            Instr::Prim {
                id: _,
                ty: _,
                op: PrimOp::Nequal,
                attrs: _,
                params,
                loc: _,
            } => (state.get(&params[0].id()) != state.get(&params[1].id())) as i64,
            Instr::Prim {
                id: _,
                ty: _,
                op: PrimOp::Gt,
                attrs: _,
                params,
                loc: _,
            } => (state.get(&params[0].id()) > state.get(&params[1].id())) as i64,
            Instr::Prim {
                id: _,
                ty: _,
                op: PrimOp::Lt,
                attrs: _,
                params,
                loc: _,
            } => (state.get(&params[0].id()) < state.get(&params[1].id())) as i64,
            Instr::Prim {
                id: _,
                ty: _,
                op: PrimOp::Ge,
                attrs: _,
                params,
                loc: _,
            } => (state.get(&params[0].id()) >= state.get(&params[1].id())) as i64,
            Instr::Prim {
                id: _,
                ty: _,
                op: PrimOp::Le,
                attrs: _,
                params,
                loc: _,
            } => (state.get(&params[0].id()) <= state.get(&params[1].id())) as i64,
        }
    }
}
