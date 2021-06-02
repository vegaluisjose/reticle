use crate::ast::*;
use std::collections::HashMap;

fn build_env(prog: &Prog) -> HashMap<String, Ty> {
    let mut env: HashMap<String, Ty> = HashMap::new();
    let inp: Vec<ExprTerm> = prog.sig().input().clone().into();
    // add prog inputs to environment
    for e in inp {
        if let Some(id) = e.id() {
            if let Some(ty) = e.ty() {
                env.insert(id, ty.clone());
            }
        }
    }
    // add instr outputs to environment
    for instr in prog.body() {
        let dst: Vec<ExprTerm> = instr.dst().clone().into();
        for e in dst {
            if let Some(id) = e.id() {
                if let Some(ty) = e.ty() {
                    env.insert(id, ty.clone());
                }
            }
        }
    }
    env
}

pub fn infer_type_try_from_prog(prog: &Prog) -> Prog {
    let env = build_env(prog);
    let mut prog = prog.clone();
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
    prog
}
