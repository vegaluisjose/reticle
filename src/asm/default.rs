use crate::asm::ast::*;

impl Default for Prog {
    fn default() -> Prog {
        Prog {
            sig: Signature::default(),
            body: Vec::new(),
        }
    }
}
