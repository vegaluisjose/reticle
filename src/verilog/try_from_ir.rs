use crate::ir::ast as ir;
use crate::util::errors::Error;
use crate::verilog::ast as vl;
use crate::verilog::constant;
// use std::collections::HashSet;
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
        // let mut decl: Vec<vl::Decl> = Vec::new();
        // for instr in def.body() {
        //     let d: Vec<vl::Decl> = instr.clone().try_into()?;
        //     decl.extend(d);
        // }
        // let decl_set: HashSet<vl::Decl> = decl.into_iter().collect();
        // let output: Vec<vl::Decl> = wire_try_from_expr(def.sig().output().clone())?;
        // let output_set: HashSet<vl::Decl> = output.into_iter().collect();
        // let mut module = vl::Module::try_from(def.sig().clone())?;
        // for decl in decl_set.difference(&output_set) {
        //     module.add_decl(decl.clone());
        // }
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
