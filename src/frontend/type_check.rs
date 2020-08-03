use crate::lang::ast::*;
use std::collections::HashMap;

// this is a "quick-dirty" type checking for resolving
// types of references (expressions)

type Env = HashMap<String, Ty>;

fn build_env_from_def(def: &Def) -> Env {
    let mut env = Env::new();
    // do this only for inputs, since outputs are defined in the body
    for input in def.inputs().iter() {
        if let Some(ty) = env.insert(input.id(), input.ty().clone()) {
            panic!(
                "Error: input id {} was already defined with type {}",
                input.id(),
                ty
            );
        }
    }
    for instr in def.body().iter() {
        if let Some(ty) = env.insert(instr.id(), instr.ty().clone()) {
            panic!(
                "Error: instr id {} was already defined with type {}",
                instr.id(),
                ty
            );
        }
    }
    env
}

fn change_expr(input: &Expr, ty: Ty) -> Expr {
    match input {
        Expr::Ref(n, Ty::Hole) => Expr::Ref(n.to_string(), ty),
        _ => panic!("Error: not a reference"),
    }
}

fn change_instr(input: &Instr, env: &Env) -> Instr {
    let mut instr = input.clone();
    instr.clear_params();
    for e in input.params().iter() {
        if let Some(ty) = env.get(&e.id()) {
            instr.add_param(change_expr(e, ty.clone()));
        } else {
            panic!("Error");
        }
    }
    instr
}

pub fn type_check(input: &Prog) -> Prog {
    let mut prog = Prog::default();
    for d in input.defs().iter() {
        let env = build_env_from_def(d);
        let mut def = Def::new_with_signature(d.signature().clone());
        for i in d.body().iter() {
            def.add_instr(change_instr(i, &env));
        }
        prog.add_def(def);
    }
    prog
}