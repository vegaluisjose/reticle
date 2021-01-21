use crate::ir::ast::*;
use crate::util::errors::Error;
use std::collections::HashMap;

fn build_env(def: &Def) -> Result<HashMap<String, Ty>, Error> {
    let mut env: HashMap<String, Ty> = HashMap::new();
    let inp: Vec<ExprTerm> = def.sig().input().clone().into();
    // add def inputs to environment
    for e in inp {
        if let Some(id) = e.id() {
            if let Some(ty) = e.ty() {
                env.insert(id, ty.clone());
            }
        }
    }
    // add instr outputs to environment
    for instr in def.body() {
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

fn infer_type_try_from_def(def: Def) -> Result<Def, Error> {
    let env = build_env(&def)?;
    let mut def = def;
    // sort
    def.sort_body()?;
    // solve instr arg types with environment
    for instr in def.body_mut() {
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
    Ok(def)
}

pub fn infer_type_try_from_prog(prog: Prog) -> Result<Prog, Error> {
    let mut res = Prog::default();
    for (name, def) in prog.def() {
        res.insert(name, infer_type_try_from_def(def.clone())?);
    }
    Ok(res)
}
