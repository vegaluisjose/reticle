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
            "Error: support only one definition atm"
        );
        if let Some(def) = self.defs().iter().next() {
            for instr in def.body().iter() {
                instr.check();
            }
        }
    }
}
