use crate::backend::asm::ast::*;

impl Expr {
    pub fn new_ref(name: &str, ty: Ty) -> Expr {
        Expr::Ref(name.to_string(), ty)
    }
}

impl Instr {
    pub fn set_dst(&mut self, name: &str) {
        self.dst = Some(name.to_string());
    }

    pub fn add_param(&mut self, expr: Expr) {
        self.params.push(expr);
    }
}

impl Prog {
    pub fn new(sig: Sig) -> Prog {
        Prog {
            sig,
            body: Vec::new(),
        }
    }

    pub fn id(&self) -> String {
        self.sig.id()
    }

    pub fn add_instr(&mut self, instr: Instr) {
        self.body.push(instr);
    }

    pub fn body(&self) -> &Vec<Instr> {
        &self.body
    }
}
