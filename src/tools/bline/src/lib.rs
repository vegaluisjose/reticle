pub mod errors;

use crate::errors::Error;
use ir::ast as ir;
use itertools::izip;
use std::collections::HashSet;
use std::convert::TryFrom;
use std::convert::TryInto;
use verilog::ast as vl;

const CLOCK: &str = "clock";
const RESET: &str = "reset";

fn vec_expr_try_from_term(term: &ir::ExprTerm) -> Result<Vec<vl::Expr>, Error> {
    match term {
        ir::ExprTerm::Any => Ok(vec![vl::Expr::new_ref("")]),
        ir::ExprTerm::Var(id, ty) => {
            let mut exprs: Vec<vl::Expr> = Vec::new();
            if let Some(length) = ty.length() {
                for n in 0..length {
                    let name = format!("{}_{}", id, n);
                    exprs.push(vl::Expr::new_ref(&name));
                }
            } else {
                exprs.push(vl::Expr::new_ref(id));
            }
            Ok(exprs)
        }
        _ => Err(Error::new_bline_error("not implemented yet")),
    }
}

#[allow(dead_code)]
fn vec_expr_try_from_tup(tup: &ir::ExprTup) -> Result<Vec<vl::Expr>, Error> {
    let mut exprs: Vec<vl::Expr> = Vec::new();
    for t in tup.term() {
        exprs.extend(vec_expr_try_from_term(t)?);
    }
    Ok(exprs)
}

#[allow(dead_code)]
fn vec_expr_try_from_expr(expr: &ir::Expr) -> Result<Vec<vl::Expr>, Error> {
    match expr {
        ir::Expr::Tup(tup) => Ok(vec_expr_try_from_tup(tup)?),
        ir::Expr::Term(term) => Ok(vec_expr_try_from_term(term)?),
    }
}

fn sign_expr_try_from_term(term: ir::ExprTerm) -> Result<Vec<vl::Expr>, Error> {
    match term {
        ir::ExprTerm::Var(id, ty) => {
            let mut exprs: Vec<vl::Expr> = Vec::new();
            if let Some(length) = ty.length() {
                for n in 0..length {
                    let name = format!("{}_{}", id, n);
                    if ty.is_signed() {
                        exprs.push(vl::Expr::new_signed_ref(&name));
                    } else {
                        exprs.push(vl::Expr::new_ref(&name));
                    }
                }
            } else if ty.is_signed() {
                exprs.push(vl::Expr::new_signed_ref(id));
            } else {
                exprs.push(vl::Expr::new_ref(id));
            }
            Ok(exprs)
        }
        _ => Err(Error::new_bline_error("not implemented yet")),
    }
}

fn wire_try_from_term(term: &ir::ExprTerm) -> Result<Vec<vl::Decl>, Error> {
    let mut decls: Vec<vl::Decl> = Vec::new();
    if let Some(width) = term.width() {
        let exprs: Vec<vl::Expr> = vec_expr_try_from_term(term)?;
        for e in exprs {
            let d = vl::Decl::new_wire(&e.id(), width);
            decls.push(d);
        }
    }
    Ok(decls)
}

fn wire_try_from_tup(tup: &ir::ExprTup) -> Result<Vec<vl::Decl>, Error> {
    let mut decls: Vec<vl::Decl> = Vec::new();
    for term in tup.term() {
        let t: Vec<vl::Decl> = wire_try_from_term(term)?;
        decls.extend(t);
    }
    Ok(decls)
}

fn wire_try_from_expr(expr: &ir::Expr) -> Result<Vec<vl::Decl>, Error> {
    match expr {
        ir::Expr::Tup(tup) => Ok(wire_try_from_tup(tup)?),
        ir::Expr::Term(term) => Ok(wire_try_from_term(term)?),
    }
}

fn reg_try_from_term(term: &ir::ExprTerm) -> Result<Vec<vl::Decl>, Error> {
    let mut decls: Vec<vl::Decl> = Vec::new();
    if let Some(width) = term.width() {
        let exprs: Vec<vl::Expr> = vec_expr_try_from_term(term)?;
        for e in exprs {
            let d = vl::Decl::new_reg(&e.id(), width);
            decls.push(d);
        }
    }
    Ok(decls)
}

fn reg_try_from_tup(tup: &ir::ExprTup) -> Result<Vec<vl::Decl>, Error> {
    let mut decls: Vec<vl::Decl> = Vec::new();
    for term in tup.term() {
        let t: Vec<vl::Decl> = reg_try_from_term(term)?;
        decls.extend(t);
    }
    Ok(decls)
}

fn reg_try_from_expr(expr: &ir::Expr) -> Result<Vec<vl::Decl>, Error> {
    match expr {
        ir::Expr::Tup(tup) => Ok(reg_try_from_tup(tup)?),
        ir::Expr::Term(term) => Ok(reg_try_from_term(term)?),
    }
}

fn input_try_from_sig(sig: ir::Sig) -> Result<Vec<vl::Port>, Error> {
    let mut port: Vec<vl::Port> = Vec::new();
    port.push(vl::Port::Input(vl::Decl::new_wire(CLOCK, 1)));
    port.push(vl::Port::Input(vl::Decl::new_wire(RESET, 1)));
    let input: Vec<vl::Decl> = wire_try_from_expr(sig.input())?;
    for decl in input {
        port.push(vl::Port::Input(decl.clone()));
    }
    Ok(port)
}

fn vec_decl_try_from_instr_wire(instr: &ir::InstrWire) -> Result<Vec<vl::Decl>, Error> {
    Ok(wire_try_from_expr(instr.dst())?)
}

fn vec_decl_try_from_instr_prim(instr: &ir::InstrPrim) -> Result<Vec<vl::Decl>, Error> {
    match instr.op() {
        ir::OpPrim::Reg => Ok(reg_try_from_expr(instr.dst())?),
        _ => Ok(wire_try_from_expr(instr.dst())?),
    }
}

fn vec_decl_try_from_instr(instr: &ir::Instr) -> Result<Vec<vl::Decl>, Error> {
    match instr {
        ir::Instr::Wire(instr) => Ok(vec_decl_try_from_instr_wire(instr)?),
        ir::Instr::Prim(instr) => Ok(vec_decl_try_from_instr_prim(instr)?),
        _ => Err(Error::new_bline_error("call not implemented yet")),
    }
}

fn vec_stmt_try_from_instr_prim(instr: &ir::InstrPrim) -> Result<Vec<vl::Stmt>, Error> {
    match instr.op() {
        ir::OpPrim::Reg => {
            let attr: Vec<i32> = instr.attr().clone().try_into()?;
            if let Some(d0) = instr.dst().idx(0) {
                if let Some(a0) = instr.arg().idx(0) {
                    if let Some(en) = instr.arg().idx(1) {
                        let v0: i32 = if let Some(v) = attr.get(0) { *v } else { 0 };
                        let event = vl::Sequential::new_posedge(CLOCK);
                        let rst_expr = vl::Expr::new_ref(RESET);
                        let ena_id: vl::Id = en.clone().try_into()?;
                        let ena_expr = vl::Expr::new_ref(&ena_id);
                        let dst: Vec<vl::Expr> = vec_expr_try_from_term(d0)?;
                        let arg: Vec<vl::Expr> = vec_expr_try_from_term(a0)?;
                        let val_expr = vl::Expr::new_int(v0);
                        let mut stmt: Vec<vl::Stmt> = Vec::new();
                        for (d, a) in dst.iter().zip(arg.iter()) {
                            let mut always = vl::ParallelProcess::new_always();
                            let s0 = vl::Sequential::new_nonblk_assign(d.clone(), val_expr.clone());
                            let s1 = vl::Sequential::new_nonblk_assign(d.clone(), a.clone());
                            let mut i0 = vl::SequentialIfElse::new(rst_expr.clone());
                            let mut i1 = vl::SequentialIfElse::new(ena_expr.clone());
                            i0.add_seq(s0);
                            i1.add_seq(s1);
                            i0.set_else(i1.into());
                            always.set_event(event.clone());
                            always.add_seq(i0.into());
                            stmt.push(vl::Stmt::from(always));
                        }
                        Ok(stmt)
                    } else {
                        Err(Error::new_bline_error("reg instr must have en arg"))
                    }
                } else {
                    Err(Error::new_bline_error("reg instr must have two args"))
                }
            } else {
                Err(Error::new_bline_error("reg instr must have one dst"))
            }
        }
        ir::OpPrim::Add => {
            if let Some(d0) = instr.dst().idx(0) {
                if let Some(a0) = instr.arg().idx(0) {
                    if let Some(a1) = instr.arg().idx(1) {
                        let d_expr: Vec<vl::Expr> = vec_expr_try_from_term(d0)?;
                        let a_expr: Vec<vl::Expr> = vec_expr_try_from_term(a0)?;
                        let b_expr: Vec<vl::Expr> = vec_expr_try_from_term(a1)?;
                        let mut stmt: Vec<vl::Stmt> = Vec::new();
                        for (d, a, b) in izip!(d_expr, a_expr, b_expr) {
                            let add = vl::Expr::new_add(a, b);
                            stmt.push(vl::Stmt::from(vl::Parallel::Assign(d, add)));
                        }
                        Ok(stmt)
                    } else {
                        Err(Error::new_bline_error("add instr must have en arg"))
                    }
                } else {
                    Err(Error::new_bline_error("add instr must have two args"))
                }
            } else {
                Err(Error::new_bline_error("add instr must have one dst"))
            }
        }
        ir::OpPrim::Mul => {
            if let Some(d0) = instr.dst().idx(0) {
                if let Some(a0) = instr.arg().idx(0) {
                    if let Some(a1) = instr.arg().idx(1) {
                        let d_expr: Vec<vl::Expr> = vec_expr_try_from_term(d0)?;
                        let a_expr = sign_expr_try_from_term(a0.clone())?;
                        let b_expr = sign_expr_try_from_term(a1.clone())?;
                        let mut stmt: Vec<vl::Stmt> = Vec::new();
                        for (d, a, b) in izip!(d_expr, a_expr, b_expr) {
                            let add = vl::Expr::new_mul(a, b);
                            stmt.push(vl::Stmt::from(vl::Parallel::Assign(d, add)));
                        }
                        Ok(stmt)
                    } else {
                        Err(Error::new_bline_error("mul instr must have en arg"))
                    }
                } else {
                    Err(Error::new_bline_error("mul instr must have two args"))
                }
            } else {
                Err(Error::new_bline_error("mul instr must have one dst"))
            }
        }
        ir::OpPrim::Eql => {
            let term_y = instr.dst().get_term(0)?;
            let term_a = instr.arg().get_term(0)?;
            let term_b = instr.arg().get_term(1)?;
            let expr_y: Vec<vl::Expr> = vec_expr_try_from_term(term_y)?;
            let expr_a: Vec<vl::Expr> = vec_expr_try_from_term(term_a)?;
            let expr_b: Vec<vl::Expr> = vec_expr_try_from_term(term_b)?;
            let mut stmt: Vec<vl::Stmt> = Vec::new();
            for (y, a, b) in izip!(expr_y, expr_a, expr_b) {
                let eq = vl::Expr::new_eq(a, b);
                stmt.push(vl::Stmt::from(vl::Parallel::Assign(y, eq)));
            }
            Ok(stmt)
        }
        ir::OpPrim::And => {
            let term_y = instr.dst().get_term(0)?;
            let term_a = instr.arg().get_term(0)?;
            let term_b = instr.arg().get_term(1)?;
            let expr_y: Vec<vl::Expr> = vec_expr_try_from_term(term_y)?;
            let expr_a: Vec<vl::Expr> = vec_expr_try_from_term(term_a)?;
            let expr_b: Vec<vl::Expr> = vec_expr_try_from_term(term_b)?;
            let mut stmt: Vec<vl::Stmt> = Vec::new();
            for (y, a, b) in izip!(expr_y, expr_a, expr_b) {
                let eq = vl::Expr::new_bit_and(a, b);
                stmt.push(vl::Stmt::from(vl::Parallel::Assign(y, eq)));
            }
            Ok(stmt)
        }
        ir::OpPrim::Mux => {
            let term_y = instr.dst().get_term(0)?;
            let term_s = instr.arg().get_term(0)?;
            let term_a = instr.arg().get_term(1)?;
            let term_b = instr.arg().get_term(2)?;
            let expr_y: Vec<vl::Expr> = vec_expr_try_from_term(term_y)?;
            let expr_s: Vec<vl::Expr> = vec_expr_try_from_term(term_s)?;
            let expr_a: Vec<vl::Expr> = vec_expr_try_from_term(term_a)?;
            let expr_b: Vec<vl::Expr> = vec_expr_try_from_term(term_b)?;
            if let Some(s) = expr_s.get(0) {
                let mut stmt: Vec<vl::Stmt> = Vec::new();
                for (y, a, b) in izip!(expr_y, expr_a, expr_b) {
                    let eq = vl::Expr::new_mux(s.clone(), a, b);
                    stmt.push(vl::Stmt::from(vl::Parallel::Assign(y, eq)));
                }
                Ok(stmt)
            } else {
                Err(Error::new_bline_error("mux sel do not have right type"))
            }
        }
        _ => Err(Error::new_bline_error("comp op not implemented yet")),
    }
}

fn vec_stmt_try_from_instr_wire(instr: &ir::InstrWire) -> Result<Vec<vl::Stmt>, Error> {
    match instr.op() {
        // TODO: impl vector constant
        // TOOD: impl negative constant
        ir::OpWire::Con => {
            let dst = instr.dst().get_term(0)?;
            let ty = dst.get_ty()?;
            let dst: Vec<vl::Expr> = vec_expr_try_from_term(dst)?;
            let mut stmt: Vec<vl::Stmt> = Vec::new();
            if dst.len() == 1 {
                let attr = instr.attr().get_term(0)?;
                let val = u32::try_from(attr.get_val()?)?;
                if let Some(width) = ty.width() {
                    let width = u32::try_from(width)?;
                    let num = vl::Expr::new_ulit_dec(width, &val.to_string());
                    stmt.push(vl::Stmt::from(vl::Parallel::Assign(dst[0].clone(), num)));
                    Ok(stmt)
                } else {
                    Err(Error::new_bline_error("type does not have width"))
                }
            } else {
                Err(Error::new_bline_error("vector not supported yet"))
            }
        }
        ir::OpWire::Id => {
            let term_y = instr.dst().get_term(0)?;
            let term_a = instr.arg().get_term(0)?;
            let expr_y: Vec<vl::Expr> = vec_expr_try_from_term(term_y)?;
            let expr_a: Vec<vl::Expr> = vec_expr_try_from_term(term_a)?;
            let mut stmt: Vec<vl::Stmt> = Vec::new();
            for (y, a) in izip!(expr_y, expr_a) {
                stmt.push(vl::Stmt::from(vl::Parallel::Assign(y, a)));
            }
            Ok(stmt)
        }
        _ => Err(Error::new_bline_error("wire op not implemented yet")),
    }
}

fn vec_stmt_try_from_instr(instr: &ir::Instr) -> Result<Vec<vl::Stmt>, Error> {
    match instr {
        ir::Instr::Prim(instr) => Ok(vec_stmt_try_from_instr_prim(instr)?),
        ir::Instr::Wire(instr) => Ok(vec_stmt_try_from_instr_wire(instr)?),
        ir::Instr::Call(_) => Err(Error::new_bline_error("call instr not implemented yet")),
    }
}

pub fn behav_try_from_ir_def(def: &ir::Def) -> Result<vl::Module, Error> {
    let id = def.sig().id();
    let mut module = vl::Module::new(&id);
    let input = input_try_from_sig(def.sig().clone())?;
    for i in input {
        module.add_port(i.clone());
    }
    let output: Vec<ir::ExprTerm> = def.sig().output().clone().into();
    let output_set: HashSet<ir::ExprTerm> = output.into_iter().collect();
    let mut stmt: Vec<vl::Stmt> = Vec::new();
    for instr in def.body() {
        let dst: Vec<ir::ExprTerm> = instr.dst().clone().into();
        let decl = vec_decl_try_from_instr(instr)?;
        for e in dst {
            for d in &decl {
                if output_set.contains(&e) {
                    module.add_port(vl::Port::Output(d.clone()));
                } else {
                    module.add_decl(d.clone())
                }
            }
        }
        let s: Vec<vl::Stmt> = vec_stmt_try_from_instr(instr)?;
        stmt.extend(s);
    }
    for s in stmt {
        module.add_stmt(s);
    }
    Ok(module)
}

pub fn behav_try_from_ir_prog(prog: &ir::Prog) -> Result<vl::Module, Error> {
    if let Some(def) = prog.get("main") {
        Ok(behav_try_from_ir_def(def)?)
    } else {
        Err(Error::new_bline_error("main not found"))
    }
}
