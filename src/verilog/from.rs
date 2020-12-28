use crate::ir::ast as ir;
use crate::ml::ast as ml;
use crate::util::errors::Error;
use crate::verilog::ast as verilog;
use std::collections::HashSet;
use std::convert::TryFrom;

mod ml_to_verilog {
    use super::*;

    pub fn expr_try_from_basc_arg(instr: &ml::InstrBasc) -> Result<verilog::Expr, Error> {
        match instr.op() {
            ml::OpBasc::Ext => {
                if let Some(attr) = instr.attr().tup() {
                    if let Some(idx) = attr.idx(0).val() {
                        if let Ok(udx) = usize::try_from(idx) {
                            if let Some(arg) = instr.arg().tup() {
                                let ids: Vec<verilog::Id> = arg.clone().into();
                                if let Some(ty) = arg.idx(0).ty() {
                                    if ty.is_vector() {
                                        Ok(verilog::Expr::new_ref(&ids[udx]))
                                    } else {
                                        Ok(verilog::Expr::new_index_bit(&ids[0], udx as i32))
                                    }
                                } else {
                                    Err(Error::new_conv_error("arg is not var expr"))
                                }
                            } else {
                                Err(Error::new_conv_error("term arg not implemented"))
                            }
                        } else {
                            Err(Error::new_conv_error("index overflow"))
                        }
                    } else {
                        Err(Error::new_conv_error("attr is not a value"))
                    }
                } else {
                    Err(Error::new_conv_error("attr must be a tuple"))
                }
            }
            _ => Err(Error::new_conv_error("not implemented yet")),
        }
    }

    pub fn expr_try_from_arg(instr: &ml::Instr) -> Result<verilog::Expr, Error> {
        match instr {
            ml::Instr::Basc(instr) => Ok(expr_try_from_basc_arg(instr)?),
            _ => Err(Error::new_conv_error("not implemented yet")),
        }
    }

    pub fn expr_try_from_dst(instr: &ml::Instr) -> Result<verilog::Expr, Error> {
        match instr {
            ml::Instr::Basc(instr) => {
                if let Some(dst) = instr.dst().term() {
                    if let Some(id) = dst.id() {
                        Ok(verilog::Expr::new_ref(&id))
                    } else {
                        Err(Error::new_conv_error("arg is not var expr"))
                    }
                } else {
                    Err(Error::new_conv_error("tup dst not supported"))
                }
            }
            _ => Err(Error::new_conv_error("not implemented yet")),
        }
    }
}

impl From<ir::ExprTerm> for Vec<verilog::Id> {
    fn from(term: ir::ExprTerm) -> Self {
        let mut ids: Vec<verilog::Id> = Vec::new();
        if let Some(ty) = term.ty() {
            if let Some(id) = term.id() {
                if let Some(length) = ty.length() {
                    for i in 0..length {
                        ids.push(format!("{}_{}", id, i));
                    }
                } else {
                    ids.push(id);
                }
            }
        }
        ids
    }
}

impl From<ir::ExprTup> for Vec<verilog::Id> {
    fn from(tup: ir::ExprTup) -> Self {
        let mut ids: Vec<verilog::Id> = Vec::new();
        for t in tup.term() {
            let i: Vec<verilog::Id> = t.clone().into();
            ids.extend(i);
        }
        ids
    }
}

impl From<ir::ExprTerm> for Vec<verilog::Decl> {
    fn from(term: ir::ExprTerm) -> Self {
        let ids: Vec<verilog::Id> = term.clone().into();
        let mut decls: Vec<verilog::Decl> = Vec::new();
        if let Some(ty) = term.ty() {
            if let Some(width) = ty.width() {
                for id in ids {
                    let wire = verilog::Decl::new_wire(&id, width);
                    decls.push(wire);
                }
            }
        }
        decls
    }
}

impl From<ir::ExprTup> for Vec<verilog::Decl> {
    fn from(tup: ir::ExprTup) -> Self {
        let mut decls: Vec<verilog::Decl> = Vec::new();
        for term in tup.term() {
            let t: Vec<verilog::Decl> = term.clone().into();
            decls.extend(t);
        }
        decls
    }
}

impl From<ir::Expr> for Vec<verilog::Decl> {
    fn from(expr: ir::Expr) -> Self {
        match &expr {
            ir::Expr::Tup(tup) => tup.clone().into(),
            ir::Expr::Term(term) => term.clone().into(),
        }
    }
}

impl From<ir::InstrWire> for Vec<verilog::Decl> {
    fn from(instr: ir::InstrWire) -> Self {
        instr.dst().clone().into()
    }
}

impl From<ir::Instr> for Vec<verilog::Decl> {
    fn from(instr: ir::Instr) -> Self {
        match &instr {
            ir::Instr::Wire(instr) => instr.clone().into(),
            _ => unimplemented!(),
        }
    }
}

impl From<ir::Sig> for verilog::Module {
    fn from(sig: ir::Sig) -> Self {
        let id = sig.id();
        let mut module = verilog::Module::new(&id);
        let input: Vec<verilog::Decl> = sig.input().clone().into();
        let output: Vec<verilog::Decl> = sig.output().clone().into();
        for i in input {
            module.add_port(verilog::Port::Input(i.clone()))
        }
        for o in output {
            module.add_port(verilog::Port::Output(o.clone()))
        }
        module
    }
}

impl From<ir::Def> for verilog::Module {
    fn from(def: ir::Def) -> Self {
        let mut module = verilog::Module::from(def.sig().clone());
        let output: Vec<verilog::Decl> = def.sig().output().clone().into();
        let output_set: HashSet<verilog::Decl> = output.into_iter().collect();
        let mut instr: Vec<verilog::Decl> = Vec::new();
        for i in def.body() {
            let d: Vec<verilog::Decl> = i.clone().into();
            instr.extend(d);
        }
        let instr_set: HashSet<verilog::Decl> = instr.into_iter().collect();
        for d in instr_set.difference(&output_set) {
            module.add_decl(d.clone());
        }
        module
    }
}

impl From<ml::InstrBasc> for Vec<verilog::Decl> {
    fn from(instr: ml::InstrBasc) -> Self {
        instr.dst().clone().into()
    }
}

impl From<ml::Instr> for Vec<verilog::Decl> {
    fn from(instr: ml::Instr) -> Self {
        match &instr {
            ml::Instr::Basc(instr) => instr.clone().into(),
            _ => unimplemented!(),
        }
    }
}

impl TryFrom<ml::Instr> for verilog::Stmt {
    type Error = Error;
    fn try_from(instr: ml::Instr) -> Result<Self, Self::Error> {
        let rval = ml_to_verilog::expr_try_from_arg(&instr)?;
        let lval = ml_to_verilog::expr_try_from_dst(&instr)?;
        Ok(verilog::Stmt::from(verilog::Parallel::Assign(lval, rval)))
    }
}

impl TryFrom<ml::Prog> for verilog::Module {
    type Error = Error;
    fn try_from(prog: ml::Prog) -> Result<Self, Self::Error> {
        let mut module = verilog::Module::from(prog.sig().clone());
        let output: Vec<verilog::Decl> = prog.sig().output().clone().into();
        let output_set: HashSet<verilog::Decl> = output.into_iter().collect();
        let mut decl: Vec<verilog::Decl> = Vec::new();
        for i in prog.body() {
            let d: Vec<verilog::Decl> = i.clone().into();
            module.add_stmt(verilog::Stmt::try_from(i.clone())?);
            decl.extend(d);
        }
        let decl_set: HashSet<verilog::Decl> = decl.into_iter().collect();
        for d in decl_set.difference(&output_set) {
            module.add_decl(d.clone());
        }
        Ok(module)
    }
}
