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

impl TryFrom<ir::ExprTerm> for Vec<vl::Decl> {
    type Error = Error;
    fn try_from(term: ir::ExprTerm) -> Result<Self, Self::Error> {
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
}

impl TryFrom<ir::ExprTup> for Vec<vl::Decl> {
    type Error = Error;
    fn try_from(tup: ir::ExprTup) -> Result<Self, Self::Error> {
        let mut decls: Vec<vl::Decl> = Vec::new();
        for term in tup.term() {
            let t: Vec<vl::Decl> = term.clone().try_into()?;
            decls.extend(t);
        }
        Ok(decls)
    }
}

impl TryFrom<ir::Expr> for Vec<vl::Decl> {
    type Error = Error;
    fn try_from(expr: ir::Expr) -> Result<Self, Self::Error> {
        match expr {
            ir::Expr::Tup(tup) => Ok(tup.try_into()?),
            ir::Expr::Term(term) => Ok(term.try_into()?),
        }
    }
}

impl TryFrom<ir::InstrWire> for Vec<vl::Decl> {
    type Error = Error;
    fn try_from(instr: ir::InstrWire) -> Result<Self, Self::Error> {
        Ok(instr.dst().clone().try_into()?)
    }
}

impl TryFrom<ir::Instr> for Vec<vl::Decl> {
    type Error = Error;
    fn try_from(instr: ir::Instr) -> Result<Self, Self::Error> {
        match instr {
            ir::Instr::Wire(instr) => Ok(instr.try_into()?),
            _ => Err(Error::new_conv_error("not implemented yet")),
        }
    }
}

impl TryFrom<ir::Sig> for vl::Module {
    type Error = Error;
    fn try_from(sig: ir::Sig) -> Result<Self, Self::Error> {
        let id = sig.id();
        let mut module = vl::Module::new(&id);
        let input: Vec<vl::Decl> = sig.input().clone().try_into()?;
        let output: Vec<vl::Decl> = sig.output().clone().try_into()?;
        module.add_input(constant::CLOCK, 1);
        module.add_input(constant::RESET, 1);
        for decl in input {
            module.add_port(vl::Port::Input(decl.clone()))
        }
        for decl in output {
            module.add_port(vl::Port::Output(decl.clone()))
        }
        Ok(module)
    }
}

impl TryFrom<ir::Def> for vl::Module {
    type Error = Error;
    fn try_from(def: ir::Def) -> Result<Self, Self::Error> {
        let mut decl: Vec<vl::Decl> = Vec::new();
        for instr in def.body() {
            let d: Vec<vl::Decl> = instr.clone().try_into()?;
            decl.extend(d);
        }
        let decl_set: HashSet<vl::Decl> = decl.into_iter().collect();
        let output: Vec<vl::Decl> = def.sig().output().clone().try_into()?;
        let output_set: HashSet<vl::Decl> = output.into_iter().collect();
        let mut module = vl::Module::try_from(def.sig().clone())?;
        for decl in decl_set.difference(&output_set) {
            module.add_decl(decl.clone());
        }
        Ok(module)
    }
}
