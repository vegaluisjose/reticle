use crate::asm::ast::*;
use std::collections::HashMap;

// Resolve types for references

type Env = HashMap<String, Ty>;

// build environment
fn build_env(prog: &Prog) -> Env {
    let mut env = Env::new();
    // add inputs
    for input in prog.inputs().iter() {
        if let Some(ty) = env.insert(input.id(), input.ty().clone()) {
            panic!(
                "Error: input id {} was already defined with type {}",
                input.id(),
                ty
            );
        }
    }
    // add instr values
    for instr in prog.body().iter() {
        if let Some(ty) = env.insert(instr.dst_id(), instr.dst_ty().clone()) {
            panic!(
                "Error: instr id {} was already defined with type {}",
                instr.dst_id(),
                ty
            );
        }
    }
    env
}

// infer reference expression
fn infer_expr(instr: &Expr, ty: Ty) -> Expr {
    match instr {
        Expr::Ref(n, Ty::Hole) => Expr::Ref(n.to_string(), ty),
        _ => panic!("Error: not a reference"),
    }
}

// infer instruction
fn infer_instr(instr: &Instr, env: &Env) -> Instr {
    let mut new_instr = instr.clone();
    new_instr.clear_params();
    for e in instr.params().iter() {
        if let Some(ty) = env.get(&e.id()) {
            new_instr.add_param(infer_expr(e, ty.clone()));
        } else {
            panic!("Error");
        }
    }
    new_instr
}

// infer program
pub fn infer_prog(prog: &Prog) -> Prog {
    let env = build_env(prog);
    let mut new_prog = Prog::new_with_signature(prog.signature().clone());
    for instr in prog.body().iter() {
        new_prog.add_instr(infer_instr(instr, &env));
    }
    new_prog
}
