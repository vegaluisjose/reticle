use crate::lang::ast::*;

impl Default for Sig {
    fn default() -> Sig {
        Sig {
            id: String::new(),
            inputs: Vec::new(),
            outputs: Vec::new(),
        }
    }
}

impl Default for Def {
    fn default() -> Def {
        Def {
            sig: Sig::default(),
            body: Vec::new(),
        }
    }
}

impl Default for Prog {
    fn default() -> Prog {
        Prog { defs: Vec::new() }
    }
}
