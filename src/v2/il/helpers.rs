use crate::v2::il::ast::*;
use std::collections::HashMap;

impl Sig {
    pub fn id(&self) -> String {
        self.id.to_string()
    }
}

impl Def {
    pub fn id(&self) -> String {
        self.sig.id()
    }

    pub fn sig(&self) -> &Sig {
        &self.sig
    }
}

impl Prog {
    pub fn defs(&self) -> &HashMap<Id, Def> {
        &self.defs
    }

    pub fn add(&mut self, name: &str, def: Def) {
        self.defs.insert(name.to_string(), def);
    }
}
