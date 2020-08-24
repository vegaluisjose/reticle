use crate::lang::ast::*;
use std::collections::HashSet;

// this trait implements the invariant of instructions and programs in Reticle
// so it can be invoked anywhere to check reticle programs
// Still WIP
pub trait Check {
    fn check(&self);
}

#[allow(clippy::single_match)]
impl Check for Instr {
    fn check(&self) {
        match self {
            Instr::Prim {
                id,
                ty,
                op: PrimOp::Add,
                attrs,
                params,
                loc: _,
            } => {
                assert!(
                    ty == params[0].ty(),
                    "Error: [add] result type and param[0] type must match"
                );
                assert!(
                    ty == params[1].ty(),
                    "Error: [add] result type and param[1] type must match"
                );
                assert!(params.len() == 2, "Error: [add] support only two params");
                assert!(attrs.is_empty(), "Error: [add] does not support attr");
                assert!(!id.is_empty(), "Error: [add] must have an id");
            }
            _ => (),
        }
    }
}

impl Check for Prog {
    fn check(&self) {
        assert!(
            self.defs().len() == 1,
            "Error: support only one definition per program"
        );
        let mut env: HashSet<String> = HashSet::new();
        let mut unresolved: Vec<Instr> = Vec::new();
        let mut registers: Vec<Instr> = Vec::new();
        if let Some(def) = self.defs().iter().next() {
            // check instr and add registers
            for instr in def.body().iter() {
                instr.check();
                if instr.is_reg() {
                    env.insert(instr.id());
                    registers.push(instr.clone());
                }
            }
            // add inputs to env
            for input in def.inputs().iter() {
                env.insert(input.id());
            }
            // check what instruction can execute
            for instr in def.body().iter() {
                if !instr.is_reg() {
                    let mut ready = true;
                    for param in instr.params().iter() {
                        if !env.contains(&param.id()) {
                            unresolved.push(instr.clone());
                            ready = false;
                            break;
                        }
                    }
                    if ready {
                        env.insert(instr.id());
                    }
                }
            }
            // run unresolved instructions
            for instr in unresolved.iter() {
                for param in instr.params().iter() {
                    assert!(
                        env.contains(&param.id()),
                        format!(
                            "Error: undefined behavior, instruction with cycles ~~> {}",
                            instr
                        )
                    );
                }
                env.insert(instr.id());
            }
            // register instruction must have params in env
            for instr in registers.iter() {
                for param in instr.params().iter() {
                    assert!(
                        env.contains(&param.id()),
                        format!(
                            "Error: undefined behavior, instruction with cycles ~~> {}",
                            instr
                        )
                    );
                }
            }
        }
    }
}
