use crate::lang::ast::Prog;
use crate::lang::interp::eval::Eval;
use crate::lang::interp::state::State;
use crate::lang::interp::trace::Trace;

pub fn interpreter(prog: &Prog, trace: &Trace) {
    assert!(trace.is_valid(), "Error: invalid trace, check values");
    assert!(prog.defs().len() == 1, "Error: single-def support atm");
    let mut trace = trace.clone();
    let mut state = State::default();
    if let Some(def) = prog.defs().iter().next() {
        for cycle in 0..trace.len() {
            for input in def.inputs().iter() {
                state.add_input(&input.id(), trace.deq(&input.id()))
            }
            for instr in def.body().iter() {
                println!("cycle:{} value:{}", cycle, instr.eval_current(&state));
            }
        }
    }
}
