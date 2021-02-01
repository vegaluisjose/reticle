use crate::tdl::ast::*;
use crate::util::errors::Error;
use std::collections::HashMap;

type Env = HashMap<Id, Ty>;

macro_rules! try_build_env {
    ($inp:tt) => {{
        let mut env = Env::new();
        let inputs: Vec<ExprTerm> = $inp.input().clone().into();
        // add inputs to environment
        for e in inputs {
            let id = e.get_id()?;
            let ty = e.get_ty()?;
            env.insert(id, ty.clone());
        }
        // add instr outputs to environment
        for instr in $inp.body() {
            let dst: Vec<ExprTerm> = instr.dst().clone().into();
            for e in dst {
                let id = e.get_id()?;
                let ty = e.get_ty()?;
                env.insert(id, ty.clone());
            }
        }
        Ok(env)
    } as Result<Env, Error>};
}

macro_rules! try_infer_body {
    ($inp:tt, $env:tt, $ty:tt) => {{
        let mut nbody: Vec<$ty> = Vec::new();
        for instr in $inp.body() {
            let mut ninstr = instr.clone();
            let mut narg = ExprTup::default();
            let arg: Vec<ExprTerm> = instr.arg().clone().into();
            for e in arg {
                let id = e.get_id()?;
                if let Some(ty) = $env.get(&id) {
                    let term = ExprTerm::Var(id, ty.clone());
                    narg.add_term(term);
                }
            }
            ninstr.set_arg(Expr::from(narg));
            nbody.push(ninstr);
        }
        Ok(nbody)
    } as Result<Vec<$ty>, Error>};
}

macro_rules! try_infer {
    ($inp:tt, $ty:tt) => {{
        let env = try_build_env!($inp)?;
        let body = try_infer_body!($inp, env, $ty)?;
        let mut res = $inp.clone();
        res.set_body(body);
        Ok(res)
    }};
}

pub fn try_infer_prim(pat: &Pat) -> Pat {
    let mut nbody: Vec<PatInstr> = Vec::new();
    for instr in pat.body() {
        let ninstr = match instr {
            PatInstr::Comp(i) if *i.prim() == Prim::Any => {
                let mut t = i.clone();
                t.set_prim(pat.prim().clone());
                PatInstr::from(t)
            }
            _ => instr.clone(),
        };
        nbody.push(ninstr);
    }
    let mut npat = pat.clone();
    npat.set_body(nbody);
    npat
}

pub fn try_infer_pat(pat: &Pat) -> Result<Pat, Error> {
    try_infer!(pat, PatInstr)
}

pub fn try_infer_imp(imp: &Imp) -> Result<Imp, Error> {
    try_infer!(imp, ImpInstr)
}
