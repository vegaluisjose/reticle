use crate::lang::ast::{Instr, Prog};
use crate::lang::interp::eval::Eval;
use crate::lang::interp::state::State;
use crate::lang::interp::trace::Trace;

#[derive(Clone, Debug)]
pub struct Interpreter {
    print: bool,
    finished: bool,
    malformed: bool,
    failed: bool,
}

impl Default for Interpreter {
    fn default() -> Interpreter {
        Interpreter {
            print: false,
            finished: false,
            malformed: false,
            failed: false,
        }
    }
}

impl Interpreter {
    pub fn is_malformed(&self) -> bool {
        self.malformed
    }

    pub fn is_finished(&self) -> bool {
        self.finished
    }

    pub fn is_failed(&self) -> bool {
        self.failed
    }

    pub fn with_print(&mut self) -> &mut Interpreter {
        self.print = true;
        self
    }

    pub fn run(&mut self, prog: &Prog, trace: &Trace) -> &mut Interpreter {
        assert!(trace.is_valid(), "Error: invalid trace, check values");
        assert!(prog.defs().len() == 1, "Error: single-def support atm");
        let mut trace = trace.clone();
        let mut curr = State::default();
        let mut next = State::default();
        // eval one def in prog
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
                // instr queue for vars not present in state
                let mut unresolved: Vec<Instr> = Vec::new();
                // run ready-to-execute instructions, or instructions with
                // params available in the environment(state)
                for instr in def.body().iter() {
                    if instr.is_ready(&curr) {
                        let value = instr.eval(&curr);
                        if instr.is_reg() {
                            next.add_reg(&instr.id(), value);
                        } else {
                            curr.add_temp(&instr.id(), value);
                        }
                    } else {
                        unresolved.push(instr.clone());
                    }
                }
                // run remaining instructions
                for instr in unresolved.iter() {
                    if instr.is_ready(&curr) {
                        let value = instr.eval(&curr);
                        if instr.is_reg() {
                            next.add_reg(&instr.id(), value);
                        } else {
                            curr.add_temp(&instr.id(), value);
                        }
                    } else {
                        self.malformed = true;
                        break;
                    }
                }
                if !self.is_malformed() {
                    // check output, if not malformed
                    for output in def.outputs().iter() {
                        let exp = trace.deq(&output.id());
                        // store results depending on whether they are regs or temps
                        let res = if curr.is_reg(&output.id()) {
                            curr.get_reg(&output.id())
                        } else {
                            curr.get_temp(&output.id())
                        };
                        if self.print {
                            println!(
                                "[cycle:{}] [id:{}] res:{} exp:{}",
                                cycle,
                                output.id(),
                                res,
                                exp
                            );
                        }
                        if exp != res {
                            self.failed = true;
                        }
                    }
                    curr.update_regs_from_state(&next);
                } else {
                    self.failed = true;
                }
            }
            self.finished = true;
        }
        self
    }
}
