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
        if self.is_std() {
            match self.std().op() {
                StdOp::Identity => state.contains(&self.indexed_param(0).id()),
                StdOp::Const => true,
                StdOp::ShiftLeft => state.contains(&self.indexed_param(0).id()),
                StdOp::ShiftRight => state.contains(&self.indexed_param(0).id()),
            }
        } else {
            match self.prim().op() {
                PrimOp::Not => state.contains(&self.indexed_param(0).id()),
                PrimOp::Mux => {
                    state.contains(&self.indexed_param(0).id())
                        && state.contains(&self.indexed_param(1).id())
                        && state.contains(&self.indexed_param(2).id())
                }
                _ => {
                    state.contains(&self.indexed_param(0).id())
                        && state.contains(&self.indexed_param(1).id())
                }
            }
        }
    }

    fn eval(&self, state: &State) -> Value {
        if self.is_std() {
            match self.std().op() {
                StdOp::Identity => state.get(&self.indexed_param(0).id()),
                StdOp::Const => {
                    if self.dst_ty().is_vector() {
                        let val: Vec<i64> = self.attrs().iter().map(|x| x.value()).collect();
                        mask(Value::from(val), self.dst_ty())
                    } else {
                        Value::new_scalar(self.indexed_attr(0).value())
                    }
                }
                StdOp::ShiftLeft => {
                    let val: Vec<i64> = self.attrs().iter().map(|x| x.value()).collect();
                    let lhs = mask(state.get(&self.indexed_param(0).id()), self.dst_ty());
                    if self.dst_ty().is_vector() {
                        let rhs = mask(Value::from(val), self.dst_ty());
                        let mut res = Value::new_vector();
                        for (a, b) in lhs.get_vector().iter().zip(rhs.get_vector().iter()) {
                            res.push(a << b);
                        }
                        res
                    } else {
                        let rhs = mask(Value::new_scalar(val[0]), self.dst_ty());
                        Value::new_scalar(lhs.get_scalar() << rhs.get_scalar())
                    }
                }
                StdOp::ShiftRight => {
                    let val: Vec<i64> = self.attrs().iter().map(|x| x.value()).collect();
                    let lhs = mask(state.get(&self.indexed_param(0).id()), self.dst_ty());
                    if self.dst_ty().is_vector() {
                        let rhs = mask(Value::from(val), self.dst_ty());
                        let mut res = Value::new_vector();
                        for (a, b) in lhs.get_vector().iter().zip(rhs.get_vector().iter()) {
                            res.push(a >> b);
                        }
                        res
                    } else {
                        let rhs = mask(Value::new_scalar(val[0]), self.dst_ty());
                        Value::new_scalar(lhs.get_scalar() >> rhs.get_scalar())
                    }
                }
            }
        } else {
            match self.prim().op() {
                PrimOp::Reg => {
                    let en = mask(
                        state.get(&self.indexed_param(1).id()),
                        &self.indexed_param(1).ty(),
                    );
                    if en == Value::new_scalar(1) {
                        mask(state.get(&self.indexed_param(0).id()), self.dst_ty())
                    } else {
                        mask(state.get(&self.dst_id()), self.dst_ty())
                    }
                }
                PrimOp::Add => {
                    let lhs = mask(state.get(&self.indexed_param(0).id()), self.dst_ty());
                    let rhs = mask(state.get(&self.indexed_param(1).id()), self.dst_ty());
                    if self.dst_ty().is_vector() {
                        let mut res = Value::new_vector();
                        for (a, b) in lhs.get_vector().iter().zip(rhs.get_vector().iter()) {
                            res.push(a + b);
                        }
                        res
                    } else {
                        Value::new_scalar(lhs.get_scalar() + rhs.get_scalar())
                    }
                }
                PrimOp::Sub => {
                    let lhs = mask(state.get(&self.indexed_param(0).id()), self.dst_ty());
                    let rhs = mask(state.get(&self.indexed_param(1).id()), self.dst_ty());
                    if self.dst_ty().is_vector() {
                        let mut res = Value::new_vector();
                        for (a, b) in lhs.get_vector().iter().zip(rhs.get_vector().iter()) {
                            res.push(a - b);
                        }
                        res
                    } else {
                        Value::new_scalar(lhs.get_scalar() - rhs.get_scalar())
                    }
                }
                PrimOp::Mul => {
                    let lhs = mask(state.get(&self.indexed_param(0).id()), self.dst_ty());
                    let rhs = mask(state.get(&self.indexed_param(1).id()), self.dst_ty());
                    if self.dst_ty().is_vector() {
                        let mut res = Value::new_vector();
                        for (a, b) in lhs.get_vector().iter().zip(rhs.get_vector().iter()) {
                            res.push(a * b);
                        }
                        res
                    } else {
                        Value::new_scalar(lhs.get_scalar() * rhs.get_scalar())
                    }
                }
                PrimOp::Not => {
                    let input = mask(state.get(&self.indexed_param(0).id()), self.dst_ty());
                    if self.dst_ty().is_vector() {
                        let mut res = Value::new_vector();
                        for i in input.get_vector().iter() {
                            res.push(!i);
                        }
                        res
                    } else {
                        Value::new_scalar(!input.get_scalar())
                    }
                }
                PrimOp::And => {
                    let lhs = mask(state.get(&self.indexed_param(0).id()), self.dst_ty());
                    let rhs = mask(state.get(&self.indexed_param(1).id()), self.dst_ty());
                    if self.dst_ty().is_vector() {
                        let mut res = Value::new_vector();
                        for (a, b) in lhs.get_vector().iter().zip(rhs.get_vector().iter()) {
                            res.push(a & b);
                        }
                        res
                    } else {
                        Value::new_scalar(lhs.get_scalar() & rhs.get_scalar())
                    }
                }
                PrimOp::Nand => {
                    let lhs = mask(state.get(&self.indexed_param(0).id()), self.dst_ty());
                    let rhs = mask(state.get(&self.indexed_param(1).id()), self.dst_ty());
                    if self.dst_ty().is_vector() {
                        let mut res = Value::new_vector();
                        for (a, b) in lhs.get_vector().iter().zip(rhs.get_vector().iter()) {
                            res.push(!(a & b));
                        }
                        res
                    } else {
                        Value::new_scalar(!(lhs.get_scalar() & rhs.get_scalar()))
                    }
                }
                PrimOp::Or => {
                    let lhs = mask(state.get(&self.indexed_param(0).id()), self.dst_ty());
                    let rhs = mask(state.get(&self.indexed_param(1).id()), self.dst_ty());
                    if self.dst_ty().is_vector() {
                        let mut res = Value::new_vector();
                        for (a, b) in lhs.get_vector().iter().zip(rhs.get_vector().iter()) {
                            res.push(a | b);
                        }
                        res
                    } else {
                        Value::new_scalar(lhs.get_scalar() | rhs.get_scalar())
                    }
                }
                PrimOp::Nor => {
                    let lhs = mask(state.get(&self.indexed_param(0).id()), self.dst_ty());
                    let rhs = mask(state.get(&self.indexed_param(1).id()), self.dst_ty());
                    if self.dst_ty().is_vector() {
                        let mut res = Value::new_vector();
                        for (a, b) in lhs.get_vector().iter().zip(rhs.get_vector().iter()) {
                            res.push(!(a | b));
                        }
                        res
                    } else {
                        Value::new_scalar(!(lhs.get_scalar() | rhs.get_scalar()))
                    }
                }
                PrimOp::Xor => {
                    let lhs = mask(state.get(&self.indexed_param(0).id()), self.dst_ty());
                    let rhs = mask(state.get(&self.indexed_param(1).id()), self.dst_ty());
                    if self.dst_ty().is_vector() {
                        let mut res = Value::new_vector();
                        for (a, b) in lhs.get_vector().iter().zip(rhs.get_vector().iter()) {
                            res.push(a ^ b);
                        }
                        res
                    } else {
                        Value::new_scalar(lhs.get_scalar() ^ rhs.get_scalar())
                    }
                }
                PrimOp::Xnor => {
                    let lhs = mask(state.get(&self.indexed_param(0).id()), self.dst_ty());
                    let rhs = mask(state.get(&self.indexed_param(1).id()), self.dst_ty());
                    if self.dst_ty().is_vector() {
                        let mut res = Value::new_vector();
                        for (a, b) in lhs.get_vector().iter().zip(rhs.get_vector().iter()) {
                            res.push(!(a ^ b));
                        }
                        res
                    } else {
                        Value::new_scalar(!(lhs.get_scalar() ^ rhs.get_scalar()))
                    }
                }
                PrimOp::Mux => {
                    let en = mask(
                        state.get(&self.indexed_param(0).id()),
                        &self.indexed_param(0).ty(),
                    );
                    if en == Value::new_scalar(1) {
                        mask(state.get(&self.indexed_param(1).id()), self.dst_ty())
                    } else {
                        mask(state.get(&self.indexed_param(2).id()), self.dst_ty())
                    }
                }
                PrimOp::Equal => Value::new_scalar(
                    (mask(
                        state.get(&self.indexed_param(0).id()),
                        &self.indexed_param(0).ty(),
                    ) == mask(
                        state.get(&self.indexed_param(1).id()),
                        &self.indexed_param(1).ty(),
                    )) as i64,
                ),
                PrimOp::NotEqual => Value::new_scalar(
                    (mask(
                        state.get(&self.indexed_param(0).id()),
                        &self.indexed_param(0).ty(),
                    ) != mask(
                        state.get(&self.indexed_param(1).id()),
                        &self.indexed_param(1).ty(),
                    )) as i64,
                ),
                PrimOp::Gt => Value::new_scalar(
                    (mask(
                        state.get(&self.indexed_param(0).id()),
                        &self.indexed_param(0).ty(),
                    )
                    .get_scalar()
                        > mask(
                            state.get(&self.indexed_param(1).id()),
                            &self.indexed_param(1).ty(),
                        )
                        .get_scalar()) as i64,
                ),
                PrimOp::Lt => Value::new_scalar(
                    (mask(
                        state.get(&self.indexed_param(0).id()),
                        &self.indexed_param(0).ty(),
                    )
                    .get_scalar()
                        < mask(
                            state.get(&self.indexed_param(1).id()),
                            &self.indexed_param(1).ty(),
                        )
                        .get_scalar()) as i64,
                ),
                PrimOp::Ge => Value::new_scalar(
                    (mask(
                        state.get(&self.indexed_param(0).id()),
                        &self.indexed_param(0).ty(),
                    )
                    .get_scalar()
                        >= mask(
                            state.get(&self.indexed_param(1).id()),
                            &self.indexed_param(1).ty(),
                        )
                        .get_scalar()) as i64,
                ),
                PrimOp::Le => Value::new_scalar(
                    (mask(
                        state.get(&self.indexed_param(0).id()),
                        &self.indexed_param(0).ty(),
                    )
                    .get_scalar()
                        <= mask(
                            state.get(&self.indexed_param(1).id()),
                            &self.indexed_param(1).ty(),
                        )
                        .get_scalar()) as i64,
                ),
            }
        }
    }
}
