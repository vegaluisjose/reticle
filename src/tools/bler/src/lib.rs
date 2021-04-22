pub mod errors;

use crate::errors::Error;
use asm::ast as asm;
use std::collections::HashMap;
use std::path::Path;
use xim::ast as xim;
use xir::ast as xir;

type Scope = HashMap<xir::ExprTerm, xir::ExprTerm>;

pub fn scope_from_expr(left: &xir::Expr, right: &xir::Expr) -> Scope {
    let mut scope = Scope::new();
    let left: Vec<xir::ExprTerm> = left.clone().into();
    let right: Vec<xir::ExprTerm> = right.clone().into();
    for (l, r) in left.iter().zip(right.iter()) {
        scope.insert(l.clone(), r.clone());
    }
    scope
}

#[derive(Clone, Debug)]
pub struct Assembler {
    pub count: u64,
    pub prefix: String,
    pub sig: xir::Sig,
    pub body: Vec<xir::Instr>,
    pub map: HashMap<String, String>,
    pub target: xim::Target,
}

impl Default for Assembler {
    fn default() -> Self {
        Assembler {
            count: 0,
            prefix: "t".to_string(),
            sig: xir::Sig::default(),
            body: Vec::new(),
            map: HashMap::new(),
            target: xim::Target::default(),
        }
    }
}

impl Assembler {
    pub fn new(sig: xir::Sig) -> Assembler {
        let mut assembler = Assembler::default();
        let input: Vec<xir::ExprTerm> = sig.input().clone().into();
        let output: Vec<xir::ExprTerm> = sig.output().clone().into();
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
        assembler.set_sig(sig);
        assembler
    }
    pub fn sig(&self) -> &xir::Sig {
        &self.sig
    }
    pub fn body(&self) -> &Vec<xir::Instr> {
        &self.body
    }
    pub fn get_var(&self, key: &str) -> Option<&String> {
        self.map.get(key)
    }
    pub fn get_target_imp(&self, key: &str) -> Option<&xim::Imp> {
        self.target.get(key)
    }
    pub fn set_target(&mut self, target: xim::Target) {
        self.target = target;
    }
    pub fn new_var(&mut self) -> String {
        let tmp = self.count;
        self.count += 1;
        format!("{}{}", self.prefix, tmp)
    }
    pub fn add_var(&mut self, name: &str) {
        self.map.insert(name.to_string(), name.to_string());
    }
    pub fn add_instr(&mut self, instr: xir::Instr) {
        self.body.push(instr);
    }
    pub fn set_sig(&mut self, sig: xir::Sig) {
        self.sig = sig;
    }
    pub fn set_prefix(&mut self, prefix: &str) {
        self.prefix = prefix.to_string();
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
    pub fn expand_instr_id(&mut self, instr: &asm::InstrWire) -> Result<(), Error> {
        let arg = self.rename_expr(instr.arg())?;
        let dst = self.rename_expr(instr.dst())?;
        let instr = xir::InstrBasc {
            op: xir::OpBasc::Id,
            attr: xir::Expr::default(),
            dst,
            arg,
        };
        self.add_instr(xir::Instr::from(instr));
        Ok(())
    }
    pub fn expand_instr_const(&mut self, instr: &asm::InstrWire) -> Result<(), Error> {
        let attr_term = instr.attr().get_term(0)?;
        let value = attr_term.get_val()?;
        let dst_term = instr.dst().get_term(0)?;
        let mut arg_tup = xir::ExprTup::default();
        if let Some(width) = dst_term.width() {
            for i in 0..width {
                let lsb = value >> i;
                let mask = lsb & 1;
                let op = if mask == 1 {
                    xir::OpBasc::Vcc
                } else {
                    xir::OpBasc::Gnd
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
                let ty = xir::Ty::Bool;
                let term = xir::ExprTerm::Var(name, ty);
                let dst = xir::Expr::from(term.clone());
                arg_tup.add_term(term);
                let instr_basc = xir::InstrBasc {
                    op,
                    attr: xir::Expr::default(),
                    dst,
                    arg: xir::Expr::default(),
                };
                self.add_instr(xir::Instr::from(instr_basc));
            }
            if width > 1 {
                let old_id = dst_term.get_id()?;
                let dst_id = if let Some(id) = self.get_var(&old_id) {
                    id.to_string()
                } else {
                    self.rename_var(&old_id)
                };
                let dst_ty = dst_term.get_ty()?;
                let dst = xir::Expr::from(xir::ExprTerm::Var(dst_id, dst_ty.clone()));
                let arg = xir::Expr::from(arg_tup);
                let cat = xir::InstrBasc {
                    op: xir::OpBasc::Cat,
                    attr: xir::Expr::default(),
                    dst,
                    arg,
                };
                self.add_instr(xir::Instr::from(cat));
            }
        }
        Ok(())
    }
    pub fn expand_instr_asm(&mut self, instr: &asm::InstrAsm) -> Result<(), Error> {
        let instr = self.rename_instr_asm(instr)?;
        if let Some(imp) = self.get_target_imp(&instr.op().to_string()) {
            let mut scope = scope_from_expr(&imp.output(), instr.dst());
            scope.extend(scope_from_expr(&imp.input(), instr.arg()));
            for i in imp.clone().body() {
                let arg: Vec<xir::ExprTerm> = i.arg().clone().into();
                let mut arg_tup = xir::ExprTup::default();
                for a in arg {
                    if let Some(term) = scope.get(&a) {
                        arg_tup.add_term(term.clone());
                    }
                }
                let arg_expr = xir::Expr::from(arg_tup);
                let dst: Vec<xir::ExprTerm> = i.dst().clone().into();
                let mut out: Vec<xir::ExprTerm> = Vec::new();
                for d in dst {
                    if let Some(term) = scope.get(&d) {
                        out.push(term.clone());
                    } else {
                        let id = self.new_var();
                        let ty = d.get_ty()?;
                        let new = xir::ExprTerm::Var(id, ty.clone());
                        scope.insert(d.clone(), new.clone());
                        out.push(new);
                    }
                }
                let dst_expr = if i.dst().term().is_some() {
                    if let Some(term) = out.get(0) {
                        xir::Expr::from(term.clone())
                    } else {
                        xir::Expr::default()
                    }
                } else {
                    let tup = xir::ExprTup { term: out };
                    xir::Expr::from(tup)
                };
                match i {
                    xir::Instr::Mach(mach) => {
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
                            self.add_instr(xir::Instr::from(instr_mach));
                        }
                    }
                    _ => {
                        let mut instr_xir = i.clone();
                        instr_xir.set_arg(arg_expr);
                        instr_xir.set_dst(dst_expr);
                        self.add_instr(instr_xir);
                    }
                }
            }
        }
        Ok(())
    }
}

pub fn deserialize_target_from_file(prim: &str) -> xim::Target {
    let filename = format!("{}_xim.bin", prim);
    let path = Path::new(env!("OUT_DIR")).join(filename);
    xim::Target::deserialize_from_file(path)
}

pub fn deserialize_target() -> xim::Target {
    let mut lut = deserialize_target_from_file("lut");
    let dsp = deserialize_target_from_file("dsp");
    lut.extend(dsp);
    lut
}

pub fn assemble(input: asm::Prog) -> Result<xir::Prog, Error> {
    let mut assembler = Assembler::new(input.sig().clone());
    let target = deserialize_target();
    assembler.set_target(target);
    for instr in input.body() {
        match instr {
            asm::Instr::Wire(instr) if instr.op() == &asm::OpWire::Con => {
                assembler.expand_instr_const(instr)?;
            }
            asm::Instr::Wire(instr) if instr.op() == &asm::OpWire::Id => {
                assembler.expand_instr_id(instr)?;
            }
            asm::Instr::Asm(instr) => assembler.expand_instr_asm(instr)?,
            _ => (),
        }
    }
    let mut prog = xir::Prog::default();
    prog.set_sig(assembler.sig().clone());
    prog.set_body(assembler.body().clone());
    Ok(prog)
}
