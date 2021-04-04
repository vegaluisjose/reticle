use crate::ast::*;
use crate::errors::Error;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::{HashMap, HashSet};
use std::convert::TryInto;

impl Prim {
    pub fn is_any(&self) -> bool {
        matches!(self, Prim::Any)
    }
}

impl Ty {
    pub fn width(&self) -> Option<u64> {
        match self {
            Ty::Bool => Some(1),
            Ty::UInt(w) => Some(*w),
            Ty::SInt(w) => Some(*w),
            Ty::Vector(ty, _) => ty.width(),
            _ => None,
        }
    }
    pub fn length(&self) -> Option<u64> {
        match self {
            Ty::Vector(_, l) => Some(*l),
            _ => None,
        }
    }
    pub fn is_signed(&self) -> bool {
        match self {
            Ty::SInt(_) => true,
            Ty::Vector(ty, _) => ty.is_signed(),
            _ => false,
        }
    }
    pub fn is_vector(&self) -> bool {
        matches!(self, Ty::Vector(_, _))
    }
}

impl OpCall {
    pub fn new(op: &str) -> OpCall {
        OpCall { op: op.to_string() }
    }
    pub fn op(&self) -> String {
        self.op.to_string()
    }
}

impl ExprTerm {
    pub fn is_var(&self) -> bool {
        matches!(self, ExprTerm::Var(_, _))
    }
    pub fn val(&self) -> Option<i64> {
        match self {
            ExprTerm::Val(n) => Some(*n),
            _ => None,
        }
    }
    pub fn id(&self) -> Option<Id> {
        match self {
            ExprTerm::Var(n, _) => Some(n.to_string()),
            _ => None,
        }
    }
    pub fn ty(&self) -> Option<&Ty> {
        match self {
            ExprTerm::Var(_, ty) => Some(ty),
            _ => None,
        }
    }
    pub fn width(&self) -> Option<u64> {
        match self {
            ExprTerm::Var(_, ty) => ty.width(),
            _ => None,
        }
    }
    pub fn length(&self) -> Option<u64> {
        match self {
            ExprTerm::Var(_, ty) => ty.length(),
            _ => None,
        }
    }
    pub fn is_vector(&self) -> bool {
        match self {
            ExprTerm::Var(_, ty) => ty.is_vector(),
            _ => false,
        }
    }
    pub fn get_id(&self) -> Result<Id, Error> {
        match self {
            ExprTerm::Var(n, _) => Ok(n.to_string()),
            _ => Err(Error::new_conv_error("not a term")),
        }
    }
    pub fn get_ty(&self) -> Result<&Ty, Error> {
        match self {
            ExprTerm::Var(_, ty) => Ok(ty),
            _ => Err(Error::new_conv_error("not a term")),
        }
    }
    pub fn get_val(&self) -> Result<i64, Error> {
        match self {
            ExprTerm::Val(n) => Ok(*n),
            _ => Err(Error::new_conv_error("not a value")),
        }
    }
}

impl ExprTup {
    pub fn term(&self) -> &Vec<ExprTerm> {
        &self.term
    }
    pub fn idx(&self, val: usize) -> Option<&ExprTerm> {
        self.term.get(val)
    }
    pub fn is_empty(&self) -> bool {
        self.term.is_empty()
    }
    pub fn get_term(&self, index: usize) -> Result<&ExprTerm, Error> {
        if let Some(term) = self.term.get(index) {
            Ok(term)
        } else {
            Err(Error::new_conv_error("index not found"))
        }
    }
    pub fn get_id(&self, index: usize) -> Result<Id, Error> {
        let term = self.get_term(index)?;
        Ok(term.get_id()?)
    }
    pub fn get_ty(&self, index: usize) -> Result<&Ty, Error> {
        let term = self.get_term(index)?;
        Ok(term.get_ty()?)
    }
    pub fn get_val(&self, index: usize) -> Result<i64, Error> {
        let term = self.get_term(index)?;
        Ok(term.get_val()?)
    }
    pub fn add_term(&mut self, term: ExprTerm) {
        self.term.push(term);
    }
}

impl Expr {
    pub fn is_tup(&self) -> bool {
        matches!(self, Expr::Tup(_))
    }
    pub fn is_term(&self) -> bool {
        matches!(self, Expr::Term(_))
    }
    pub fn tup(&self) -> Option<&ExprTup> {
        match self {
            Expr::Tup(e) => Some(e),
            _ => None,
        }
    }
    pub fn term(&self) -> Option<&ExprTerm> {
        match self {
            Expr::Term(e) => Some(e),
            _ => None,
        }
    }
    pub fn idx(&self, val: usize) -> Option<&ExprTerm> {
        match self {
            Expr::Tup(t) => t.idx(val),
            Expr::Term(t) if val == 0 => Some(t),
            _ => None,
        }
    }
    pub fn get_term(&self, index: usize) -> Result<&ExprTerm, Error> {
        match self {
            Expr::Tup(t) => Ok(t.get_term(index)?),
            Expr::Term(t) if index == 0 => Ok(t),
            _ => Err(Error::new_conv_error("not a term")),
        }
    }
    pub fn get_id(&self, index: usize) -> Result<Id, Error> {
        match self {
            Expr::Tup(t) => Ok(t.get_id(index)?),
            Expr::Term(t) if index == 0 => Ok(t.get_id()?),
            _ => Err(Error::new_conv_error("not a term")),
        }
    }
    pub fn get_ty(&self, index: usize) -> Result<&Ty, Error> {
        match self {
            Expr::Tup(t) => Ok(t.get_ty(index)?),
            Expr::Term(t) if index == 0 => Ok(t.get_ty()?),
            _ => Err(Error::new_conv_error("not a term")),
        }
    }
}

impl InstrCall {
    pub fn op(&self) -> &OpCall {
        &self.op
    }
    pub fn dst(&self) -> &Expr {
        &self.dst
    }
    pub fn arg(&self) -> &Expr {
        &self.arg
    }
    pub fn set_dst(&mut self, dst: Expr) {
        self.dst = dst;
    }
    pub fn set_arg(&mut self, arg: Expr) {
        self.arg = arg;
    }
}

impl InstrPrim {
    pub fn op(&self) -> &OpPrim {
        &self.op
    }
    pub fn dst(&self) -> &Expr {
        &self.dst
    }
    pub fn attr(&self) -> &Expr {
        &self.attr
    }
    pub fn arg(&self) -> &Expr {
        &self.arg
    }
    pub fn prim(&self) -> &Prim {
        &self.prim
    }
    pub fn is_reg(&self) -> bool {
        matches!(self.op(), OpPrim::Reg)
    }
    pub fn set_prim(&mut self, prim: Prim) {
        self.prim = prim;
    }
    pub fn set_dst(&mut self, dst: Expr) {
        self.dst = dst;
    }
    pub fn set_arg(&mut self, arg: Expr) {
        self.arg = arg;
    }
}

impl InstrWire {
    pub fn op(&self) -> &OpWire {
        &self.op
    }
    pub fn dst(&self) -> &Expr {
        &self.dst
    }
    pub fn attr(&self) -> &Expr {
        &self.attr
    }
    pub fn arg(&self) -> &Expr {
        &self.arg
    }
    pub fn set_dst(&mut self, dst: Expr) {
        self.dst = dst;
    }
    pub fn set_arg(&mut self, arg: Expr) {
        self.arg = arg;
    }
}

impl Instr {
    pub fn is_reg(&self) -> bool {
        match self {
            Instr::Prim(instr) => instr.is_reg(),
            _ => false,
        }
    }
    pub fn is_prim(&self) -> bool {
        matches!(self, Instr::Prim(_))
    }
    pub fn is_wire(&self) -> bool {
        matches!(self, Instr::Wire(_))
    }
    pub fn is_call(&self) -> bool {
        matches!(self, Instr::Call(_))
    }
    pub fn dst(&self) -> &Expr {
        match self {
            Instr::Prim(instr) => instr.dst(),
            Instr::Wire(instr) => instr.dst(),
            Instr::Call(instr) => instr.dst(),
        }
    }
    pub fn arg(&self) -> &Expr {
        match self {
            Instr::Prim(instr) => instr.arg(),
            Instr::Wire(instr) => instr.arg(),
            Instr::Call(instr) => instr.arg(),
        }
    }
    pub fn set_dst(&mut self, dst: Expr) {
        match self {
            Instr::Prim(instr) => instr.set_dst(dst),
            Instr::Wire(instr) => instr.set_dst(dst),
            Instr::Call(instr) => instr.set_dst(dst),
        }
    }
    pub fn set_arg(&mut self, arg: Expr) {
        match self {
            Instr::Prim(instr) => instr.set_arg(arg),
            Instr::Wire(instr) => instr.set_arg(arg),
            Instr::Call(instr) => instr.set_arg(arg),
        }
    }
}

impl Sig {
    pub fn id(&self) -> String {
        self.id.to_string()
    }
    pub fn input(&self) -> &Expr {
        &self.input
    }
    pub fn output(&self) -> &Expr {
        &self.output
    }
    pub fn set_id(&mut self, id: &str) {
        self.id = id.to_string();
    }
}

fn term_is_ready(env: &HashSet<Id>, term: &ExprTerm) -> bool {
    if let Some(id) = term.id() {
        env.contains(&id)
    } else {
        false
    }
}

fn instr_is_ready(env: &HashSet<Id>, instr: &Instr) -> bool {
    let mut is_ready = true;
    if !instr.is_reg() {
        match instr.arg() {
            Expr::Tup(tup) => {
                for term in tup.term() {
                    is_ready = term_is_ready(env, term);
                    if !is_ready {
                        break;
                    }
                }
            }
            Expr::Term(term) => {
                is_ready = term_is_ready(env, term);
            }
        }
    }
    is_ready
}

impl Def {
    pub fn id(&self) -> String {
        self.sig.id()
    }
    pub fn sig(&self) -> &Sig {
        &self.sig
    }
    pub fn input(&self) -> &Expr {
        self.sig.input()
    }
    pub fn output(&self) -> &Expr {
        self.sig.output()
    }
    pub fn body(&self) -> &Vec<Instr> {
        &self.body
    }
    pub fn set_id(&mut self, id: &str) {
        self.sig.set_id(id);
    }
    pub fn set_sig(&mut self, sig: Sig) {
        self.sig = sig;
    }
    pub fn set_body(&mut self, body: Vec<Instr>) {
        self.body = body;
    }
    pub fn body_mut(&mut self) -> &mut Vec<Instr> {
        &mut self.body
    }
    pub fn shuffle_body(&mut self) {
        self.body.shuffle(&mut thread_rng());
    }
    pub fn sort_body(&mut self) -> Result<(), Error> {
        let mut env: HashSet<Id> = HashSet::new();
        let input: Vec<Id> = self.sig.input().clone().try_into()?;
        for id in input {
            env.insert(id);
        }
        let mut p: Vec<Instr> = Vec::new();
        let mut q: Vec<Instr> = self.body.clone();
        let mut pass = true;
        while !q.is_empty() {
            let mut t: Vec<Instr> = Vec::new();
            for instr in &q {
                if instr_is_ready(&env, instr) {
                    p.push(instr.clone());
                    let dst: Vec<Id> = instr.dst().clone().try_into()?;
                    for id in dst {
                        env.insert(id);
                    }
                } else {
                    t.push(instr.clone());
                }
            }
            if q.len() == t.len() {
                pass = false;
                break;
            } else {
                q = t;
            }
        }
        if pass {
            self.body = p;
            Ok(())
        } else {
            Err(Error::new_conv_error("Sorting"))
        }
    }
}

impl Prog {
    pub fn def(&self) -> &HashMap<Id, Def> {
        &self.def
    }
    pub fn get(&self, name: &str) -> Option<&Def> {
        self.def.get(name)
    }
    pub fn get_mut(&mut self, name: &str) -> Option<&mut Def> {
        self.def.get_mut(name)
    }
    pub fn insert(&mut self, name: &str, def: Def) -> Option<Def> {
        self.def.insert(name.to_string(), def)
    }
    pub fn sort_def(&mut self) -> Result<(), Error> {
        for (_, def) in self.def.iter_mut() {
            def.sort_body()?;
        }
        Ok(())
    }
}
