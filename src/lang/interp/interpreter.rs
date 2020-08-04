use crate::lang::ast::Prog;
use crate::lang::interp::eval::Eval;
use crate::lang::interp::state::State;
use crate::lang::interp::trace::Trace;

pub fn interpreter(prog: &Prog, trace: &Trace) {
    assert!(trace.is_valid(), "Error: invalid trace, check values");
    assert!(prog.defs().len() == 1, "Error: single-def support atm");
    let mut trace = trace.clone();
    let mut curr = State::default();
    let mut next = State::default();
    if let Some(def) = prog.defs().iter().next() {
        // initialize registers with zero
        for instr in def.body().iter() {
            if instr.is_reg() {
                curr.add_reg(&instr.id(), 0);
            }
        }
        for cycle in 0..trace.len() {
            for input in def.inputs().iter() {
                curr.add_input(&input.id(), trace.deq(&input.id()))
            }
            for instr in def.body().iter() {
                let value = instr.eval_current(&curr);
                if instr.is_reg() {
                    next.add_reg(&instr.id(), value);
                } else {
                    curr.add_temp(&instr.id(), value);
                    next.add_temp(&instr.id(), value);
                }
            }
            for output in def.outputs().iter() {
                let exp = trace.deq(&output.id());
                // store results depending on whether they are regs or temps
                let res = if curr.is_reg(&output.id()) {
                    curr.get(&output.id())
                } else {
                    next.get(&output.id())
                };
                println!(
                    "[cycle:{}] [id:{}] res:{} exp:{}",
                    cycle,
                    output.id(),
                    res,
                    exp
                );
                assert_eq!(exp, res, "Error: values differ");
            }
            curr.update_regs_from_state(&next);
        }
    }
}
