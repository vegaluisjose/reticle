use crate::tdl::ast::*;
use crate::util::errors::Error;
use std::collections::HashMap;

macro_rules! try_build_env {
    ($inp:tt) => {{
        let mut env: HashMap<Id, Ty> = HashMap::new();
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
    } as Result<HashMap<Id, Ty>, Error>};
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

pub fn try_infer_pat(pat: &Pat) -> Result<Pat, Error> {
    let env = try_build_env!(pat)?;
    let body = try_infer_body!(pat, env, PatInstr)?;
    let mut pat = pat.clone();
    pat.set_body(body);
    Ok(pat)
}
