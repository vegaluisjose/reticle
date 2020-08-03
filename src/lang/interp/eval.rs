use crate::lang::ast::*;
use crate::lang::interp::state::State;

pub fn eval_instr(instr: &Instr, state: &State) {
    if let Instr::Std {
        id: _,
        ty: _,
        op: _,
        attrs: _,
        params,
    } = instr
    {
        println!("eval:{}", state.get_value(&params[0].id()));
    }
}
