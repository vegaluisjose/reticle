use crate::interp::state::State;
use crate::interp::ty::Value;
use crate::lang::ast::*;

fn mask(value: Value, ty: &Ty) -> Value {
    // mask width since we are not planning to go above 64-bit
    // gotta fix Ty at some point to reflect this
    let width = ty.width() as u32;
    let two: i64 = 2;
    match ty {
        Ty::SInt(_) if value.get_scalar() < 0 => {
            Value::new_scalar(-(value.get_scalar().abs() & (two.pow(width - 1) - 1)))
        }
        Ty::SInt(_) => Value::new_scalar(value.get_scalar() & (two.pow(width - 1) - 1)),
        Ty::UInt(_) => Value::new_scalar(value.get_scalar() & (two.pow(width) - 1)),
        Ty::Bool => Value::new_scalar(value.get_scalar() & 1),
        Ty::Vector(scalar_ty, _) => {
            let mut masked = Value::new_vector();
            for val in value.get_vector().iter() {
                let scalar = Value::new_scalar(*val);
                masked.push(mask(scalar, scalar_ty).get_scalar());
            }
            masked
        }
        _ => panic!("Error: undefined type cannot be masked"),
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
                op: StdOp::Const,
                attrs: _,
                params: _,
            } => true,
            Instr::Std {
                id: _,
                ty: _,
                op: StdOp::ShiftLeft,
                attrs: _,
                params,
            } => state.contains(&params[0].id()),
            Instr::Std {
                id: _,
                ty: _,
                op: StdOp::ShiftRight,
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
            Instr::Std {
                id: _,
                ty,
                op: StdOp::Const,
                attrs,
                params: _,
            } => {
                if ty.is_vector() {
                    let val: Vec<i64> = attrs.iter().map(|x| x.value()).collect();
                    mask(Value::from(val), ty)
                } else {
                    Value::new_scalar(attrs[0].value())
                }
            }
            Instr::Std {
                id: _,
                ty,
                op: StdOp::ShiftLeft,
                attrs,
                params,
            } => {
                let val: Vec<i64> = attrs.iter().map(|x| x.value()).collect();
                let lhs = mask(state.get(&params[0].id()), ty);
                if ty.is_vector() {
                    let rhs = mask(Value::from(val), ty);
                    let mut res = Value::new_vector();
                    for (a, b) in lhs.get_vector().iter().zip(rhs.get_vector().iter()) {
                        res.push(a << b);
                    }
                    res
                } else {
                    let rhs = mask(Value::new_scalar(val[0]), ty);
                    Value::new_scalar(lhs.get_scalar() << rhs.get_scalar())
                }
            }
            Instr::Std {
                id: _,
                ty,
                op: StdOp::ShiftRight,
                attrs,
                params,
            } => {
                let val: Vec<i64> = attrs.iter().map(|x| x.value()).collect();
                let lhs = mask(state.get(&params[0].id()), ty);
                if ty.is_vector() {
                    let rhs = mask(Value::from(val), ty);
                    let mut res = Value::new_vector();
                    for (a, b) in lhs.get_vector().iter().zip(rhs.get_vector().iter()) {
                        res.push(a >> b);
                    }
                    res
                } else {
                    let rhs = mask(Value::new_scalar(val[0]), ty);
                    Value::new_scalar(lhs.get_scalar() >> rhs.get_scalar())
                }
            }
            Instr::Prim {
                id,
                ty,
                op: PrimOp::Reg,
                attrs: _,
                params,
                loc: _,
            } => {
                let en = mask(state.get(&params[1].id()), &params[1].ty());
                if en == Value::new_scalar(1) {
                    mask(state.get(&params[0].id()), ty)
                } else {
                    mask(state.get(id), ty)
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
                let lhs = mask(state.get(&params[0].id()), ty);
                let rhs = mask(state.get(&params[1].id()), ty);
                if ty.is_vector() {
                    let mut res = Value::new_vector();
                    for (a, b) in lhs.get_vector().iter().zip(rhs.get_vector().iter()) {
                        res.push(a + b);
                    }
                    res
                } else {
                    Value::new_scalar(lhs.get_scalar() + rhs.get_scalar())
                }
            }
            Instr::Prim {
                id: _,
                ty,
                op: PrimOp::Sub,
                attrs: _,
                params,
                loc: _,
            } => {
                let lhs = mask(state.get(&params[0].id()), ty);
                let rhs = mask(state.get(&params[1].id()), ty);
                if ty.is_vector() {
                    let mut res = Value::new_vector();
                    for (a, b) in lhs.get_vector().iter().zip(rhs.get_vector().iter()) {
                        res.push(a - b);
                    }
                    res
                } else {
                    Value::new_scalar(lhs.get_scalar() - rhs.get_scalar())
                }
            }
            Instr::Prim {
                id: _,
                ty,
                op: PrimOp::Mul,
                attrs: _,
                params,
                loc: _,
            } => {
                let lhs = mask(state.get(&params[0].id()), ty);
                let rhs = mask(state.get(&params[1].id()), ty);
                if ty.is_vector() {
                    let mut res = Value::new_vector();
                    for (a, b) in lhs.get_vector().iter().zip(rhs.get_vector().iter()) {
                        res.push(a * b);
                    }
                    res
                } else {
                    Value::new_scalar(lhs.get_scalar() * rhs.get_scalar())
                }
            }
            Instr::Prim {
                id: _,
                ty,
                op: PrimOp::Not,
                attrs: _,
                params,
                loc: _,
            } => {
                let input = mask(state.get(&params[0].id()), ty);
                if ty.is_vector() {
                    let mut res = Value::new_vector();
                    for i in input.get_vector().iter() {
                        res.push(!i);
                    }
                    res
                } else {
                    Value::new_scalar(!input.get_scalar())
                }
            }
            Instr::Prim {
                id: _,
                ty,
                op: PrimOp::And,
                attrs: _,
                params,
                loc: _,
            } => {
                let lhs = mask(state.get(&params[0].id()), ty);
                let rhs = mask(state.get(&params[1].id()), ty);
                if ty.is_vector() {
                    let mut res = Value::new_vector();
                    for (a, b) in lhs.get_vector().iter().zip(rhs.get_vector().iter()) {
                        res.push(a & b);
                    }
                    res
                } else {
                    Value::new_scalar(lhs.get_scalar() & rhs.get_scalar())
                }
            }
            Instr::Prim {
                id: _,
                ty,
                op: PrimOp::Nand,
                attrs: _,
                params,
                loc: _,
            } => {
                let lhs = mask(state.get(&params[0].id()), ty);
                let rhs = mask(state.get(&params[1].id()), ty);
                if ty.is_vector() {
                    let mut res = Value::new_vector();
                    for (a, b) in lhs.get_vector().iter().zip(rhs.get_vector().iter()) {
                        res.push(!(a & b));
                    }
                    res
                } else {
                    Value::new_scalar(!(lhs.get_scalar() & rhs.get_scalar()))
                }
            }
            Instr::Prim {
                id: _,
                ty,
                op: PrimOp::Or,
                attrs: _,
                params,
                loc: _,
            } => {
                let lhs = mask(state.get(&params[0].id()), ty);
                let rhs = mask(state.get(&params[1].id()), ty);
                if ty.is_vector() {
                    let mut res = Value::new_vector();
                    for (a, b) in lhs.get_vector().iter().zip(rhs.get_vector().iter()) {
                        res.push(a | b);
                    }
                    res
                } else {
                    Value::new_scalar(lhs.get_scalar() | rhs.get_scalar())
                }
            }
            Instr::Prim {
                id: _,
                ty,
                op: PrimOp::Nor,
                attrs: _,
                params,
                loc: _,
            } => {
                let lhs = mask(state.get(&params[0].id()), ty);
                let rhs = mask(state.get(&params[1].id()), ty);
                if ty.is_vector() {
                    let mut res = Value::new_vector();
                    for (a, b) in lhs.get_vector().iter().zip(rhs.get_vector().iter()) {
                        res.push(!(a | b));
                    }
                    res
                } else {
                    Value::new_scalar(!(lhs.get_scalar() | rhs.get_scalar()))
                }
            }
            Instr::Prim {
                id: _,
                ty,
                op: PrimOp::Xor,
                attrs: _,
                params,
                loc: _,
            } => {
                let lhs = mask(state.get(&params[0].id()), ty);
                let rhs = mask(state.get(&params[1].id()), ty);
                if ty.is_vector() {
                    let mut res = Value::new_vector();
                    for (a, b) in lhs.get_vector().iter().zip(rhs.get_vector().iter()) {
                        res.push(a ^ b);
                    }
                    res
                } else {
                    Value::new_scalar(lhs.get_scalar() ^ rhs.get_scalar())
                }
            }
            Instr::Prim {
                id: _,
                ty,
                op: PrimOp::Xnor,
                attrs: _,
                params,
                loc: _,
            } => {
                let lhs = mask(state.get(&params[0].id()), ty);
                let rhs = mask(state.get(&params[1].id()), ty);
                if ty.is_vector() {
                    let mut res = Value::new_vector();
                    for (a, b) in lhs.get_vector().iter().zip(rhs.get_vector().iter()) {
                        res.push(!(a ^ b));
                    }
                    res
                } else {
                    Value::new_scalar(!(lhs.get_scalar() ^ rhs.get_scalar()))
                }
            }
            Instr::Prim {
                id: _,
                ty,
                op: PrimOp::Mux,
                attrs: _,
                params,
                loc: _,
            } => {
                if ty.is_vector() {
                    panic!("Error: vector mux is not supported");
                } else {
                    let en = mask(state.get(&params[0].id()), &params[0].ty());
                    if en == Value::new_scalar(1) {
                        mask(state.get(&params[1].id()), ty)
                    } else {
                        mask(state.get(&params[2].id()), ty)
                    }
                }
            }
            Instr::Prim {
                id: _,
                ty,
                op: PrimOp::Equal,
                attrs: _,
                params,
                loc: _,
            } => {
                if ty.is_vector() {
                    panic!("Error: vector eq is not supported");
                } else {
                    Value::new_scalar(
                        (mask(state.get(&params[0].id()), &params[0].ty())
                            == mask(state.get(&params[1].id()), &params[1].ty()))
                            as i64,
                    )
                }
            }
            Instr::Prim {
                id: _,
                ty,
                op: PrimOp::NotEqual,
                attrs: _,
                params,
                loc: _,
            } => {
                if ty.is_vector() {
                    panic!("Error: vector neq is not supported");
                } else {
                    Value::new_scalar(
                        (mask(state.get(&params[0].id()), &params[0].ty())
                            != mask(state.get(&params[1].id()), &params[1].ty()))
                            as i64,
                    )
                }
            }
            Instr::Prim {
                id: _,
                ty,
                op: PrimOp::Gt,
                attrs: _,
                params,
                loc: _,
            } => {
                if ty.is_vector() {
                    panic!("Error: vector gt is not supported");
                } else {
                    Value::new_scalar(
                        (mask(state.get(&params[0].id()), &params[0].ty()).get_scalar()
                            > mask(state.get(&params[1].id()), &params[1].ty()).get_scalar())
                            as i64,
                    )
                }
            }
            Instr::Prim {
                id: _,
                ty,
                op: PrimOp::Lt,
                attrs: _,
                params,
                loc: _,
            } => {
                if ty.is_vector() {
                    panic!("Error: vector lt is not supported");
                } else {
                    Value::new_scalar(
                        (mask(state.get(&params[0].id()), &params[0].ty()).get_scalar()
                            < mask(state.get(&params[1].id()), &params[1].ty()).get_scalar())
                            as i64,
                    )
                }
            }
            Instr::Prim {
                id: _,
                ty,
                op: PrimOp::Ge,
                attrs: _,
                params,
                loc: _,
            } => {
                if ty.is_vector() {
                    panic!("Error: vector ge is not supported");
                } else {
                    Value::new_scalar(
                        (mask(state.get(&params[0].id()), &params[0].ty()).get_scalar()
                            >= mask(state.get(&params[1].id()), &params[1].ty()).get_scalar())
                            as i64,
                    )
                }
            }
            Instr::Prim {
                id: _,
                ty,
                op: PrimOp::Le,
                attrs: _,
                params,
                loc: _,
            } => {
                if ty.is_vector() {
                    panic!("Error: vector le is not supported");
                } else {
                    Value::new_scalar(
                        (mask(state.get(&params[0].id()), &params[0].ty()).get_scalar()
                            <= mask(state.get(&params[1].id()), &params[1].ty()).get_scalar())
                            as i64,
                    )
                }
            }
        }
    }
}