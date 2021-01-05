use crate::util::errors::Error;
use crate::xl::ast::*;
use std::collections::HashMap;

fn build_env(prog: &Prog) -> Result<HashMap<String, Ty>, Error> {
    let mut env: HashMap<String, Ty> = HashMap::new();
    let inp: Vec<ExprTerm> = prog.sig().input().clone().into();
    let mut err = false;
    // add prog inputs to environment
    for e in inp {
        if let Some(id) = e.id() {
            if let Some(ty) = e.ty() {
                env.insert(id, ty.clone());
            } else {
                err = true;
                break;
            }
        } else {
            err = true;
            break;
        }
    }
    // add instr outputs to environment
    for instr in prog.body() {
        let dst: Vec<ExprTerm> = instr.dst().clone().into();
        for e in dst {
            if let Some(id) = e.id() {
                if let Some(ty) = e.ty() {
                    env.insert(id, ty.clone());
                } else {
                    err = true;
                    break;
                }
            } else {
                err = true;
                break;
            }
        }
    }
    if err {
        Err(Error::new_type_error("terms must be vars"))
    } else {
        Ok(env)
    }
}

pub fn infer_type_try_from_prog(prog: Prog) -> Result<Prog, Error> {
    let env = build_env(&prog)?;
    let mut prog = prog;
    // solve instr arg types with environment
    for instr in prog.body_mut() {
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
    Ok(prog)
}
