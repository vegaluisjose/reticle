pub mod try_from;

use crate::asm::ast as asm;
use crate::tdl::ast as tdl;
use crate::util::errors::Error;
use crate::xl::ast as xl;
use std::collections::HashMap;

type Scope = HashMap<xl::ExprTerm, xl::ExprTerm>;

pub fn scope_from_expr(left: &xl::Expr, right: &xl::Expr) -> Scope {
    let mut scope = Scope::new();
    let left: Vec<xl::ExprTerm> = left.clone().into();
    let right: Vec<xl::ExprTerm> = right.clone().into();
    for (l, r) in left.iter().zip(right.iter()) {
        scope.insert(l.clone(), r.clone());
    }
    scope
}

#[derive(Clone, Debug)]
pub struct Assembler {
    pub count: u64,
    pub prefix: String,
    pub sig: xl::Sig,
    pub body: Vec<xl::Instr>,
    pub map: HashMap<String, String>,
    pub imp: HashMap<String, tdl::Imp>,
}

impl Default for Assembler {
    fn default() -> Self {
        Assembler {
            count: 0,
            prefix: "n".to_string(),
            sig: xl::Sig::default(),
            body: Vec::new(),
            map: HashMap::new(),
            imp: HashMap::new(),
        }
    }
}

impl Assembler {
    pub fn new(sig: &xl::Sig) -> Assembler {
        let mut assembler = Assembler::default();
        assembler.set_sig(sig.clone());
        let input: Vec<xl::ExprTerm> = sig.input().clone().into();
        let output: Vec<xl::ExprTerm> = sig.output().clone().into();
        for i in input {
            if let Some(id) = i.id() {
                assembler.add_var(&id);
            }
        }
        for o in output {
            if let Some(id) = o.id() {
                assembler.add_var(&id);
            }
        }
        assembler
    }
    pub fn sig(&self) -> &xl::Sig {
        &self.sig
    }
    pub fn body(&self) -> &Vec<xl::Instr> {
        &self.body
    }
    pub fn get_var(&self, key: &str) -> Option<&String> {
        self.map.get(key)
    }
    pub fn get_imp(&self, key: &str) -> Option<&tdl::Imp> {
        self.imp.get(key)
    }
    pub fn set_imp(&mut self, imp: HashMap<String, tdl::Imp>) {
        self.imp = imp;
    }
    pub fn new_var(&mut self) -> String {
        let tmp = self.count;
        self.count += 1;
        format!("{}{}", self.prefix, tmp)
    }
    pub fn add_var(&mut self, name: &str) {
        self.map.insert(name.to_string(), name.to_string());
    }
    pub fn rename_var(&mut self, name: &str) -> String {
        let tmp = self.new_var();
        self.map.insert(name.to_string(), tmp.clone());
        tmp
    }
    pub fn rename_term(&mut self, term: &asm::ExprTerm) -> Result<asm::ExprTerm, Error> {
        let id = if let Some(val) = self.get_var(&term.get_id()?) {
            val.to_string()
        } else {
            self.rename_var(&term.get_id()?)
        };
        let ty = term.get_ty()?;
        Ok(asm::ExprTerm::Var(id, ty.clone()))
    }
    pub fn rename_expr(&mut self, expr: &asm::Expr) -> Result<asm::Expr, Error> {
        if let Some(term) = expr.term() {
            Ok(asm::Expr::from(self.rename_term(term)?))
        } else {
            let term: Vec<asm::ExprTerm> = expr.clone().into();
            let mut tup = asm::ExprTup::default();
            for t in &term {
                tup.add_term(self.rename_term(t)?);
            }
            Ok(asm::Expr::from(tup))
        }
    }
    pub fn rename_instr_asm(&mut self, instr: &asm::InstrAsm) -> Result<asm::InstrAsm, Error> {
        let arg = self.rename_expr(instr.arg())?;
        let dst = self.rename_expr(instr.dst())?;
        let mut instr = instr.clone();
        instr.set_dst(dst);
        instr.set_arg(arg);
        Ok(instr)
    }
    pub fn expand_instr_const(&mut self, instr: &asm::InstrWire) -> Result<(), Error> {
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
                    let old_id = dst_term.get_id()?;
                    if let Some(id) = self.get_var(&old_id) {
                        id.to_string()
                    } else {
                        self.rename_var(&old_id)
                    }
                } else {
                    self.new_var()
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
                let old_id = dst_term.get_id()?;
                let dst_id = if let Some(id) = self.get_var(&old_id) {
                    id.to_string()
                } else {
                    self.rename_var(&old_id)
                };
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
    pub fn expand_instr_asm(&mut self, instr: &asm::InstrAsm) -> Result<(), Error> {
        let instr = self.rename_instr_asm(instr)?;
        if let Some(imp) = self.get_imp(&instr.op().to_string()) {
            let mut scope = scope_from_expr(&imp.output(), instr.dst());
            scope.extend(scope_from_expr(&imp.input(), instr.arg()));
            for i in imp.clone().body() {
                let arg: Vec<xl::ExprTerm> = i.arg().clone().into();
                let mut arg_tup = xl::ExprTup::default();
                for a in arg {
                    if let Some(term) = scope.get(&a) {
                        arg_tup.add_term(term.clone());
                    }
                }
                let arg_expr = xl::Expr::from(arg_tup);
                let dst: Vec<xl::ExprTerm> = i.dst().clone().into();
                let mut out: Vec<xl::ExprTerm> = Vec::new();
                for d in dst {
                    if let Some(term) = scope.get(&d) {
                        out.push(term.clone());
                    } else {
                        let id = self.new_var();
                        let ty = d.get_ty()?;
                        let new = xl::ExprTerm::Var(id, ty.clone());
                        scope.insert(d.clone(), new.clone());
                        out.push(new);
                    }
                }
                let dst_expr = if i.dst().term().is_some() {
                    if let Some(term) = out.get(0) {
                        xl::Expr::from(term.clone())
                    } else {
                        xl::Expr::default()
                    }
                } else {
                    let tup = xl::ExprTup { term: out };
                    xl::Expr::from(tup)
                };
                match i {
                    xl::Instr::Mach(mach) => {
                        if let Some(loc) = mach.loc() {
                            let mut loc = loc.clone();
                            let x = instr.loc().x().clone();
                            let y = instr.loc().y().clone();
                            loc.set_x(x);
                            loc.set_y(y);
                            let mut instr_mach = mach.clone();
                            instr_mach.set_loc(loc);
                            instr_mach.set_arg(arg_expr);
                            instr_mach.set_dst(dst_expr);
                            self.add_instr(xl::Instr::from(instr_mach));
                        }
                    }
                    _ => {
                        let mut instr_xl = i.clone();
                        instr_xl.set_arg(arg_expr);
                        instr_xl.set_dst(dst_expr);
                        self.add_instr(instr_xl);
                    }
                }
            }
        }
        Ok(())
    }
    pub fn add_instr(&mut self, instr: xl::Instr) {
        self.body.push(instr);
    }
    pub fn set_sig(&mut self, sig: xl::Sig) {
        self.sig = sig;
    }
    pub fn set_prefix(&mut self, prefix: &str) {
        self.prefix = prefix.to_string();
    }
}
