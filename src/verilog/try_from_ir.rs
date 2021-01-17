use crate::ir::ast as ir;
use crate::util::errors::Error;
use crate::verilog::ast as vl;
use crate::verilog::constant;
use std::collections::HashSet;
use std::convert::TryFrom;
use std::convert::TryInto;

impl TryFrom<ir::ExprTerm> for Vec<vl::Expr> {
    type Error = Error;
    fn try_from(term: ir::ExprTerm) -> Result<Self, Self::Error> {
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
            _ => Err(Error::new_conv_error("not implemented yet")),
        }
    }
}

impl TryFrom<ir::ExprTup> for Vec<vl::Expr> {
    type Error = Error;
    fn try_from(tup: ir::ExprTup) -> Result<Self, Self::Error> {
        let mut exprs: Vec<vl::Expr> = Vec::new();
        for t in tup.term() {
            let v: Vec<vl::Expr> = t.clone().try_into()?;
            exprs.extend(v)
        }
        Ok(exprs)
    }
}

impl TryFrom<ir::Expr> for Vec<vl::Expr> {
    type Error = Error;
    fn try_from(expr: ir::Expr) -> Result<Self, Self::Error> {
        match expr {
            ir::Expr::Tup(tup) => Ok(tup.try_into()?),
            ir::Expr::Term(term) => Ok(term.try_into()?),
        }
    }
}

fn wire_try_from_term(term: ir::ExprTerm) -> Result<Vec<vl::Decl>, Error> {
    let mut decls: Vec<vl::Decl> = Vec::new();
    if let Some(width) = term.width() {
        let exprs: Vec<vl::Expr> = term.try_into()?;
        for e in exprs {
            let d = vl::Decl::new_wire(&e.id(), width);
            decls.push(d);
        }
    }
    Ok(decls)
}

fn wire_try_from_tup(tup: ir::ExprTup) -> Result<Vec<vl::Decl>, Error> {
    let mut decls: Vec<vl::Decl> = Vec::new();
    for term in tup.term() {
        let t: Vec<vl::Decl> = wire_try_from_term(term.clone())?;
        decls.extend(t);
    }
    Ok(decls)
}

pub fn wire_try_from_expr(expr: ir::Expr) -> Result<Vec<vl::Decl>, Error> {
    match expr {
        ir::Expr::Tup(tup) => Ok(wire_try_from_tup(tup)?),
        ir::Expr::Term(term) => Ok(wire_try_from_term(term)?),
    }
}

fn reg_try_from_term(term: ir::ExprTerm) -> Result<Vec<vl::Decl>, Error> {
    let mut decls: Vec<vl::Decl> = Vec::new();
    if let Some(width) = term.width() {
        let exprs: Vec<vl::Expr> = term.try_into()?;
        for e in exprs {
            let d = vl::Decl::new_reg(&e.id(), width);
            decls.push(d);
        }
    }
    Ok(decls)
}

fn reg_try_from_tup(tup: ir::ExprTup) -> Result<Vec<vl::Decl>, Error> {
    let mut decls: Vec<vl::Decl> = Vec::new();
    for term in tup.term() {
        let t: Vec<vl::Decl> = reg_try_from_term(term.clone())?;
        decls.extend(t);
    }
    Ok(decls)
}

pub fn reg_try_from_expr(expr: ir::Expr) -> Result<Vec<vl::Decl>, Error> {
    match expr {
        ir::Expr::Tup(tup) => Ok(reg_try_from_tup(tup)?),
        ir::Expr::Term(term) => Ok(reg_try_from_term(term)?),
    }
}

impl TryFrom<ir::InstrWire> for Vec<vl::Decl> {
    type Error = Error;
    fn try_from(instr: ir::InstrWire) -> Result<Self, Self::Error> {
        Ok(wire_try_from_expr(instr.dst().clone())?)
    }
}

impl TryFrom<ir::InstrComp> for Vec<vl::Decl> {
    type Error = Error;
    fn try_from(instr: ir::InstrComp) -> Result<Self, Self::Error> {
        match instr.op() {
            ir::OpComp::Reg => Ok(reg_try_from_expr(instr.dst().clone())?),
            _ => Ok(wire_try_from_expr(instr.dst().clone())?),
        }
    }
}

impl TryFrom<ir::Instr> for Vec<vl::Decl> {
    type Error = Error;
    fn try_from(instr: ir::Instr) -> Result<Self, Self::Error> {
        match instr {
            ir::Instr::Wire(instr) => Ok(instr.try_into()?),
            ir::Instr::Comp(instr) => Ok(instr.try_into()?),
            _ => Err(Error::new_conv_error("call not implemented yet")),
        }
    }
}

impl TryFrom<ir::InstrComp> for Vec<vl::Stmt> {
    type Error = Error;
    fn try_from(instr: ir::InstrComp) -> Result<Self, Self::Error> {
        match instr.op() {
            ir::OpComp::Reg => {
                let attr: Vec<i32> = instr.attr().clone().try_into()?;
                if let Some(d0) = instr.dst().idx(0) {
                    if let Some(a0) = instr.arg().idx(0) {
                        if let Some(en) = instr.arg().idx(1) {
                            let v0: i32 = if let Some(v) = attr.get(0) { *v } else { 0 };
                            let event = vl::Sequential::new_posedge(constant::CLOCK);
                            let rst_expr = vl::Expr::new_ref(constant::RESET);
                            let ena_id: vl::Id = en.clone().try_into()?;
                            let ena_expr = vl::Expr::new_ref(&ena_id);
                            let dst: Vec<vl::Expr> = d0.clone().try_into()?;
                            let arg: Vec<vl::Expr> = a0.clone().try_into()?;
                            let val_expr = vl::Expr::new_int(v0);
                            let mut stmt: Vec<vl::Stmt> = Vec::new();
                            for (d, a) in dst.iter().zip(arg.iter()) {
                                let mut always = vl::ParallelProcess::new_always();
                                let s0 =
                                    vl::Sequential::new_nonblk_assign(d.clone(), val_expr.clone());
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
                            Err(Error::new_conv_error("reg instr must have en arg"))
                        }
                    } else {
                        Err(Error::new_conv_error("reg instr must have two args"))
                    }
                } else {
                    Err(Error::new_conv_error("reg instr must have one dst"))
                }
            }
            _ => Err(Error::new_conv_error("comp op not implemented yet")),
        }
    }
}

impl TryFrom<ir::Instr> for Vec<vl::Stmt> {
    type Error = Error;
    fn try_from(instr: ir::Instr) -> Result<Self, Self::Error> {
        match instr {
            ir::Instr::Comp(instr) => Ok(instr.try_into()?),
            ir::Instr::Wire(_) => Err(Error::new_conv_error("wire instr not implemented yet")),
            ir::Instr::Call(_) => Err(Error::new_conv_error("call instr not implemented yet")),
        }
    }
}

fn input_try_from_sig(sig: ir::Sig) -> Result<Vec<vl::Port>, Error> {
    let mut port: Vec<vl::Port> = Vec::new();
    port.push(vl::Port::Input(vl::Decl::new_wire(constant::CLOCK, 1)));
    port.push(vl::Port::Input(vl::Decl::new_wire(constant::RESET, 1)));
    let input: Vec<vl::Decl> = wire_try_from_expr(sig.input().clone())?;
    for decl in input {
        port.push(vl::Port::Input(decl.clone()));
    }
    Ok(port)
}

impl TryFrom<ir::Sig> for vl::Module {
    type Error = Error;
    fn try_from(sig: ir::Sig) -> Result<Self, Self::Error> {
        let id = sig.id();
        let mut module = vl::Module::new(&id);
        let input = input_try_from_sig(sig.clone())?;
        for port in input {
            module.add_port(port.clone())
        }
        let output: Vec<vl::Decl> = wire_try_from_expr(sig.output().clone())?;
        for decl in output {
            module.add_port(vl::Port::Output(decl.clone()))
        }
        Ok(module)
    }
}

impl TryFrom<ir::Def> for vl::Module {
    type Error = Error;
    fn try_from(def: ir::Def) -> Result<Self, Self::Error> {
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
            let decl: Vec<vl::Decl> = instr.clone().try_into()?;
            for e in dst {
                for d in &decl {
                    if instr.is_reg() && output_set.contains(&e) {
                        module.add_port(vl::Port::Output(d.clone()));
                    } else {
                        module.add_decl(d.clone())
                    }
                }
            }
            let s: Vec<vl::Stmt> = instr.clone().try_into()?;
            stmt.extend(s);
        }
        for s in stmt {
            module.add_stmt(s);
        }
        Ok(module)
    }
}

// convert main only for now
impl TryFrom<ir::Prog> for vl::Module {
    type Error = Error;
    fn try_from(prog: ir::Prog) -> Result<Self, Self::Error> {
        if let Some(def) = prog.get("main") {
            Ok(vl::Module::try_from(def.clone())?)
        } else {
            Err(Error::new_conv_error("main not found"))
        }
    }
}
