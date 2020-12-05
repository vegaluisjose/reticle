use crate::v2::il::ast::*;

impl Sig {
    pub fn id(&self) -> String {
        self.id.to_string()
    }
}

impl Def {
    pub fn id(&self) -> String {
        self.sig.id()
    }
}

impl Prog {
    pub fn add(&mut self, name: &str, def: Def) {
        self.map.insert(name.to_string(), def);
    }
}
