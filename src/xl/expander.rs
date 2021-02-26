use crate::asm::ast as asm;
use crate::tdl::ast as tdl;
use crate::util::errors::Error;
use crate::xl::ast as xl;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Expander {
    pub count: u64,
    pub prefix: String,
    pub sig: xl::Sig,
    pub body: Vec<xl::Instr>,
    pub map: HashMap<String, String>,
    pub imp: HashMap<String, tdl::Imp>,
}

impl Default for Expander {
    fn default() -> Self {
        Expander {
            count: 0,
            prefix: "t".to_string(),
            sig: xl::Sig::default(),
            body: Vec::new(),
            map: HashMap::new(),
            imp: HashMap::new(),
        }
    }
}

impl Expander {
    pub fn sig(&self) -> &xl::Sig {
        &self.sig
    }
    pub fn body(&self) -> &Vec<xl::Instr> {
        &self.body
    }
    pub fn get_name(&self, key: &str) -> Option<&String> {
        self.map.get(key)
    }
    pub fn get_imp(&self, key: &str) -> Option<&tdl::Imp> {
        self.imp.get(key)
    }
    pub fn new_name(&mut self) -> String {
        let tmp = self.count;
        self.count += 1;
        format!("{}{}", self.prefix, tmp)
    }
    pub fn rename(&mut self, name: &str) -> String {
        let tmp = self.new_name();
        self.map.insert(name.to_string(), tmp.clone());
        tmp
    }
    pub fn expand_const(&mut self, instr: &asm::InstrWire) -> Result<(), Error> {
        let attr_term = instr.attr().get_term(0)?;
        let value = attr_term.get_val()?;
        let dst_term = instr.dst().get_term(0)?;
        let mut arg_tup = xl::ExprTup::default();
        if let Some(width) = dst_term.width() {
            for i in 0..width {
                let lsb = value >> i;
                let mask = lsb & 1;
                let op = if mask == 1 {
                    xl::OpBasc::Vcc
                } else {
                    xl::OpBasc::Gnd
                };
                let name = if width == 1 {
                    self.rename(&dst_term.get_id()?)
                } else {
                    self.new_name()
                };
                let ty = xl::Ty::Bool;
                let term = xl::ExprTerm::Var(name, ty);
                let dst = xl::Expr::from(term.clone());
                arg_tup.add_term(term);
                let instr_basc = xl::InstrBasc {
                    op,
                    attr: xl::Expr::default(),
                    dst,
                    arg: xl::Expr::default(),
                };
                self.add_instr(xl::Instr::from(instr_basc));
            }
            if width > 1 {
                let dst_id = self.rename(&dst_term.get_id()?);
                let dst_ty = dst_term.get_ty()?;
                let dst = xl::Expr::from(xl::ExprTerm::Var(dst_id, dst_ty.clone()));
                let arg = xl::Expr::from(arg_tup);
                let cat = xl::InstrBasc {
                    op: xl::OpBasc::Cat,
                    attr: xl::Expr::default(),
                    dst,
                    arg,
                };
                self.add_instr(xl::Instr::from(cat));
            }
        }
        Ok(())
    }
    pub fn add_instr(&mut self, instr: xl::Instr) {
        self.body.push(instr);
    }
    pub fn set_prefix(&mut self, prefix: &str) {
        self.prefix = prefix.to_string();
    }
    pub fn set_sig(&mut self, sig: xl::Sig) {
        self.sig = sig;
    }
}
