use crate::lang::ast::*;
use std::collections::HashSet;

// -- WIP --
// this trait implements the invariant of instructions and programs in Reticle
// so it can be invoked anywhere to check reticle programs.
// TODO: impl TypeCheckErrors

pub trait Check {
    fn check(&self);
}

impl Check for InstrStd {
    fn check(&self) {
        match self.op {
            StdOp::Identity => {
                assert!(
                    self.dst_ty() == self.params[0].ty(),
                    "Error: instr dst type and first param type must match"
                );
            }
            StdOp::ShiftLeft => {
                assert!(
                    self.dst_ty() == self.params[0].ty(),
                    "Error: instr dst type and first param type must match"
                );
            }
            _ => (),
        }
    }
}

impl Check for InstrPrim {
    fn check(&self) {
        match self.op {
            PrimOp::Add | PrimOp::Sub => {
                assert!(
                    self.dst_ty() == self.params[0].ty(),
                    "Error: instr dst type and first param type must match"
                );
                assert!(
                    self.dst_ty() == self.params[1].ty(),
                    "Error: instr dst type and second param type must match"
                );
                assert!(
                    self.params.len() == 2,
                    "Error: instr support only two params"
                );
                assert!(self.attrs.is_empty(), "Error: instr does not support attr");
                assert!(!self.dst_id().is_empty(), "Error: instr must have an id");
            }
            PrimOp::Equal | PrimOp::NotEqual => {
                assert!(
                    self.dst_ty() == &Ty::Bool,
                    "Error: instr dst type must be bool"
                );
            }
            _ => (),
        }
    }
}

impl Check for Instr {
    fn check(&self) {
        assert!(self.dst().is_ref(), "Error: dst must be a reference");
        match self {
            Instr::Prim(instr) => instr.check(),
            Instr::Std(instr) => instr.check(),
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
