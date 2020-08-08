use crate::interp::state::State;
use crate::interp::ty::Value;
use crate::lang::ast::*;

fn mask_scalar(value: Value, ty: &Ty) -> Value {
    // mask width since we are not planning to go above 64-bit
    // gotta fix Ty at some point to reflect this
    let width = ty.width() as u32;
    let two: i64 = 2;
    match ty {
        Ty::SInt(_) if value < 0 => -(value & (two.pow(width - 1) - 1)),
        Ty::SInt(_) => value & (two.pow(width - 1) - 1),
        Ty::UInt(_) => value & (two.pow(width) - 1),
        Ty::Bool => value & 1,
        _ => panic!("Error: {} not scalar type", ty),
    }
}

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
            Instr::Std {
                id: _,
                ty: _,
                op: StdOp::ScalarConst,
                attrs: _,
                params: _,
            } => true,
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
                ty,
                op: StdOp::Identity,
                attrs: _,
                params,
            } => mask_scalar(state.get(&params[0].id()), ty),
            Instr::Std {
                id: _,
                ty: _,
                op: StdOp::ScalarConst,
                attrs,
                params: _,
            } => attrs[0].value(),
            Instr::Prim {
                id,
                ty,
                op: PrimOp::Reg,
                attrs: _,
                params,
                loc: _,
            } => {
                let en = mask_scalar(state.get(&params[1].id()), &params[1].ty());
                if en == 1 {
                    mask_scalar(state.get(&params[0].id()), ty)
                } else {
                    mask_scalar(state.get(id), ty)
                }
            }
            Instr::Prim {
                id: _,
                ty,
                op: PrimOp::Add,
                attrs: _,
                params,
                loc: _,
            } => {
                mask_scalar(state.get(&params[0].id()), ty)
                    + mask_scalar(state.get(&params[1].id()), ty)
            }
            Instr::Prim {
                id: _,
                ty,
                op: PrimOp::Sub,
                attrs: _,
                params,
                loc: _,
            } => {
                mask_scalar(state.get(&params[0].id()), ty)
                    - mask_scalar(state.get(&params[1].id()), ty)
            }
            Instr::Prim {
                id: _,
                ty,
                op: PrimOp::Mul,
                attrs: _,
                params,
                loc: _,
            } => {
                mask_scalar(state.get(&params[0].id()), ty)
                    * mask_scalar(state.get(&params[1].id()), ty)
            }
            Instr::Prim {
                id: _,
                ty,
                op: PrimOp::Not,
                attrs: _,
                params,
                loc: _,
            } => !mask_scalar(state.get(&params[0].id()), ty),
            Instr::Prim {
                id: _,
                ty,
                op: PrimOp::And,
                attrs: _,
                params,
                loc: _,
            } => {
                mask_scalar(state.get(&params[0].id()), ty)
                    & mask_scalar(state.get(&params[1].id()), ty)
            }
            Instr::Prim {
                id: _,
                ty,
                op: PrimOp::Nand,
                attrs: _,
                params,
                loc: _,
            } => {
                !(mask_scalar(state.get(&params[0].id()), ty)
                    & mask_scalar(state.get(&params[1].id()), ty))
            }
            Instr::Prim {
                id: _,
                ty,
                op: PrimOp::Or,
                attrs: _,
                params,
                loc: _,
            } => {
                mask_scalar(state.get(&params[0].id()), ty)
                    | mask_scalar(state.get(&params[1].id()), ty)
            }
            Instr::Prim {
                id: _,
                ty,
                op: PrimOp::Nor,
                attrs: _,
                params,
                loc: _,
            } => {
                !(mask_scalar(state.get(&params[0].id()), ty)
                    | mask_scalar(state.get(&params[1].id()), ty))
            }
            Instr::Prim {
                id: _,
                ty,
                op: PrimOp::Xor,
                attrs: _,
                params,
                loc: _,
            } => {
                mask_scalar(state.get(&params[0].id()), ty)
                    ^ mask_scalar(state.get(&params[1].id()), ty)
            }
            Instr::Prim {
                id: _,
                ty,
                op: PrimOp::Xnor,
                attrs: _,
                params,
                loc: _,
            } => {
                !(mask_scalar(state.get(&params[0].id()), ty)
                    ^ mask_scalar(state.get(&params[1].id()), ty))
            }
            Instr::Prim {
                id: _,
                ty,
                op: PrimOp::Mux,
                attrs: _,
                params,
                loc: _,
            } => {
                let en = mask_scalar(state.get(&params[0].id()), &params[0].ty());
                if en == 1 {
                    mask_scalar(state.get(&params[1].id()), ty)
                } else {
                    mask_scalar(state.get(&params[2].id()), ty)
                }
            }
            Instr::Prim {
                id: _,
                ty: _,
                op: PrimOp::Equal,
                attrs: _,
                params,
                loc: _,
            } => {
                (mask_scalar(state.get(&params[0].id()), &params[0].ty())
                    == mask_scalar(state.get(&params[1].id()), &params[1].ty()))
                    as Value
            }
            Instr::Prim {
                id: _,
                ty: _,
                op: PrimOp::NotEqual,
                attrs: _,
                params,
                loc: _,
            } => {
                (mask_scalar(state.get(&params[0].id()), &params[0].ty())
                    != mask_scalar(state.get(&params[1].id()), &params[1].ty()))
                    as Value
            }
            Instr::Prim {
                id: _,
                ty: _,
                op: PrimOp::Gt,
                attrs: _,
                params,
                loc: _,
            } => {
                (mask_scalar(state.get(&params[0].id()), &params[0].ty())
                    > mask_scalar(state.get(&params[1].id()), &params[1].ty()))
                    as Value
            }
            Instr::Prim {
                id: _,
                ty: _,
                op: PrimOp::Lt,
                attrs: _,
                params,
                loc: _,
            } => {
                (mask_scalar(state.get(&params[0].id()), &params[0].ty())
                    < mask_scalar(state.get(&params[1].id()), &params[1].ty()))
                    as Value
            }
            Instr::Prim {
                id: _,
                ty: _,
                op: PrimOp::Ge,
                attrs: _,
                params,
                loc: _,
            } => {
                (mask_scalar(state.get(&params[0].id()), &params[0].ty())
                    >= mask_scalar(state.get(&params[1].id()), &params[1].ty()))
                    as Value
            }
            Instr::Prim {
                id: _,
                ty: _,
                op: PrimOp::Le,
                attrs: _,
                params,
                loc: _,
            } => {
                (mask_scalar(state.get(&params[0].id()), &params[0].ty())
                    <= mask_scalar(state.get(&params[1].id()), &params[1].ty()))
                    as Value
            }
        }
    }
}
