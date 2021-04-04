use crate::ast::*;
use crate::errors::Error;
use std::collections::HashMap;

fn build_env(imp: &Imp) -> Result<HashMap<String, Ty>, Error> {
    let mut env: HashMap<String, Ty> = HashMap::new();
    let inp: Vec<ExprTerm> = imp.sig().input().clone().into();
    // add imp inputs to environment
    for e in inp {
        if let Some(id) = e.id() {
            if let Some(ty) = e.ty() {
                env.insert(id, ty.clone());
            }
        }
    }
    // add instr outputs to environment
    for instr in imp.body() {
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

fn infer_type_try_from_imp(imp: &Imp) -> Result<Imp, Error> {
    let env = build_env(&imp)?;
    let mut imp = imp.clone();
    // solve instr arg types with environment
    for instr in imp.body_mut() {
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
    Ok(imp)
}

pub fn infer_type_try_from_target(target: Target) -> Result<Target, Error> {
    let mut res = Target::default();
    for (name, imp) in target.imp() {
        res.insert(name, infer_type_try_from_imp(imp)?);
    }
    Ok(res)
}
