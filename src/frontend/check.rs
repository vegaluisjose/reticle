use crate::lang::ast::*;

// this trait implements the invariant of instructions and programs in Reticle
// so it can be invoked anywhere to check reticle programs
// Still WIP
pub trait Check {
    fn check(&self);
}

impl Check for Instr {
    fn check(&self) {
        match self {
            Instr::Prim {
                id: _,
                ty,
                op: PrimOp::Add,
                attrs: _,
                params,
                loc: _,
            } => {
                assert!(
                    ty == params[0].ty(),
                    "Error: dst and param[0] does not match"
                );
            }
            _ => (),
        }
    }
}

impl Check for Prog {
    fn check(&self) {
        assert!(
            self.defs().len() == 1,
            "Error support only one definition atm"
        );
        if let Some(def) = self.defs().iter().next() {
            for instr in def.body().iter() {
                instr.check();
            }
        }
    }
}
