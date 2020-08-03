use crate::lang::ast::Prog;
use crate::lang::interp::eval::Eval;
use crate::lang::interp::state::State;
use crate::lang::interp::trace::Trace;

pub fn interpreter(prog: &Prog, trace: &Trace) {
    assert!(trace.is_valid(), "Error: invalid trace, check values");
    let mut state = State::default();
    state.add_input("a", 4);
    for def in prog.defs().iter() {
        for instr in def.body().iter() {
            println!("{}", instr.eval_current(&state));
        }
    }
}
