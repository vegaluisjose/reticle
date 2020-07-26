use crate::backend::asm::ast::*;
use crate::backend::verilog::Module;
use crate::lang::ast as reticle;

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
    pub fn new(prog: reticle::Prog, target: &str) -> Prog {
        assert_eq!(target, "ultrascale", "Error: ultrascale support only");
        Prog {
            sig: prog.defs[0].sig.clone(),
            body: Vec::new(),
            target: target.to_string(),
        }
    }

    pub fn id(&self) -> String {
        self.sig.id()
    }

    pub fn inputs(&self) -> &Vec<Port> {
        self.sig.inputs()
    }

    pub fn outputs(&self) -> &Vec<Port> {
        self.sig.outputs()
    }

    pub fn add_instr(&mut self, instr: Instr) {
        self.body.push(instr);
    }

    pub fn body(&self) -> &Vec<Instr> {
        &self.body
    }

    pub fn to_verilog(&self) -> Module {
        Module::from(self.clone())
    }
}
