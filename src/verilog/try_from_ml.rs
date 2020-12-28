use crate::ml::ast as ml;
use crate::util::errors::Error;
use crate::verilog::ast as verilog;
use std::collections::HashSet;
use std::convert::TryFrom;

fn expr_try_from_basc_arg(instr: &ml::InstrBasc) -> Result<verilog::Expr, Error> {
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

fn expr_try_from_arg(instr: &ml::Instr) -> Result<verilog::Expr, Error> {
    match instr {
        ml::Instr::Basc(instr) => Ok(expr_try_from_basc_arg(instr)?),
        _ => Err(Error::new_conv_error("not implemented yet")),
    }
}

fn expr_try_from_dst(instr: &ml::Instr) -> Result<verilog::Expr, Error> {
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

impl TryFrom<ml::Instr> for verilog::Stmt {
    type Error = Error;
    fn try_from(instr: ml::Instr) -> Result<Self, Self::Error> {
        let rval = expr_try_from_arg(&instr)?;
        let lval = expr_try_from_dst(&instr)?;
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
            decl.extend(d);
        }
        let decl_set: HashSet<verilog::Decl> = decl.into_iter().collect();
        for d in decl_set.difference(&output_set) {
            module.add_decl(d.clone());
        }
        for i in prog.body() {
            module.add_stmt(verilog::Stmt::try_from(i.clone())?);
        }
        Ok(module)
    }
}
