use crate::ast::*;
use crate::errors::Error;
use std::collections::HashMap;

fn build_env(pat: &Pat) -> Result<HashMap<String, Ty>, Error> {
    let mut env: HashMap<String, Ty> = HashMap::new();
    let inp: Vec<ExprTerm> = pat.sig().input().clone().into();
    // add pat inputs to environment
    for e in inp {
        if let Some(id) = e.id() {
            if let Some(ty) = e.ty() {
                env.insert(id, ty.clone());
            }
        }
    }
    // add instr outputs to environment
    for instr in pat.body() {
        let dst: Vec<ExprTerm> = instr.dst().clone().into();
        for e in dst {
            if let Some(id) = e.id() {
                if let Some(ty) = e.ty() {
                    env.insert(id, ty.clone());
                }
            }
        }
    }
    Ok(env)
}

fn infer_type_try_from_pat(pat: &Pat) -> Result<Pat, Error> {
    let env = build_env(&pat)?;
    let mut pat = pat.clone();
    // solve instr arg types with environment
    for instr in pat.body_mut() {
        let mut arg = ExprTup::default();
        if let Some(tup) = instr.arg().tup() {
            for e in tup.term() {
                if let Some(id) = e.id() {
                    if let Some(ty) = env.get(&id) {
                        let term = ExprTerm::Var(id.to_string(), ty.clone());
                        arg.add_term(term);
                    }
                }
            }
        }
        let e = Expr::from(arg);
        instr.set_arg(e);
    }
    Ok(pat)
}

pub fn infer_type_try_from_target(target: Target) -> Result<Target, Error> {
    let mut res = Target::default();
    for (name, pat) in target.pat() {
        res.insert(name, infer_type_try_from_pat(pat)?);
    }
    Ok(res)
}
