use crate::lang::ast::*;

impl Default for Signature {
    fn default() -> Signature {
        Signature {
            id: String::new(),
            inputs: Vec::new(),
            outputs: Vec::new(),
        }
    }
}

impl Default for Def {
    fn default() -> Def {
        Def {
            sig: Signature::default(),
            body: Vec::new(),
        }
    }
}

impl Default for Prog {
    fn default() -> Prog {
        Prog { defs: Vec::new() }
    }
}
