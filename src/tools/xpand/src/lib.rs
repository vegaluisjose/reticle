pub mod carry;
pub mod cat;
pub mod dsp;
pub mod errors;
pub mod expr;
pub mod ext;
pub mod fdre;
pub mod fdse;
pub mod gnd;
pub mod loc;
pub mod lut;
pub mod port;
pub mod vcc;

use crate::errors::Error;
use crate::gnd::Gnd;
use crate::port::Output;
use crate::vcc::Vcc;
use bline::{input_try_from_sig, vec_expr_try_from_expr, wire_try_from_expr};
use std::collections::HashSet;
use std::convert::TryInto;
use verilog::ast as vl;
use xir::ast as xir;

pub const CLOCK: &str = "clock";
pub const RESET: &str = "reset";

fn vec_decl_try_from_instr_basc(instr: &xir::InstrBasc) -> Result<Vec<vl::Decl>, Error> {
    Ok(wire_try_from_expr(instr.dst())?)
}

fn vec_decl_try_from_instr_mach(instr: &xir::InstrMach) -> Result<Vec<vl::Decl>, Error> {
    Ok(wire_try_from_expr(instr.dst())?)
}

fn vec_decl_try_from_instr(instr: &xir::Instr) -> Result<Vec<vl::Decl>, Error> {
    match instr {
        xir::Instr::Basc(instr) => Ok(vec_decl_try_from_instr_basc(instr)?),
        xir::Instr::Mach(instr) => Ok(vec_decl_try_from_instr_mach(instr)?),
    }
}

fn tmp_name_try_from_term(term: &xir::ExprTerm) -> Result<xir::Id, Error> {
    let dst: xir::Id = term.clone().try_into()?;
    Ok(format!("_{}", dst))
}

fn inst_name_try_from_instr(instr: &xir::InstrMach) -> Result<vl::Id, Error> {
    let dst: Vec<vl::Id> = instr.dst().clone().try_into()?;
    Ok(format!("__{}", dst[0]))
}

// TODO: default case must be error
fn stmt_from_mach(instr: &xir::InstrMach) -> Result<Vec<vl::Stmt>, Error> {
    match instr.op() {
        xir::OpMach::Lut2 => lut::lut2_from_mach(instr),
        xir::OpMach::Fdre => fdre::fdre_from_mach(instr),
        xir::OpMach::CarryAdd => carry::carry_from_mach(instr),
        _ => Err(Error::new_xpand_error("unsupported machine instruction")),
    }
}

// TODO: default case must be error
fn stmt_from_basc(instr: &xir::InstrBasc) -> Result<Vec<vl::Stmt>, Error> {
    match instr.op() {
        xir::OpBasc::Ext => ext::ext_from_basc(instr),
        xir::OpBasc::Cat => cat::cat_from_basc(instr),
        _ => Err(Error::new_xpand_error("unsupported basic instruction")),
    }
}

fn stmt_from_instr(instr: &xir::Instr) -> Result<Vec<vl::Stmt>, Error> {
    match instr {
        xir::Instr::Basc(basc) => Ok(stmt_from_basc(basc)?),
        xir::Instr::Mach(mach) => Ok(stmt_from_mach(mach)?),
    }
}

pub fn try_from_xir_prog(prog: &xir::Prog) -> Result<vl::Module, Error> {
    let id = prog.sig().id();
    let mut module = vl::Module::new(&id);
    let input = input_try_from_sig(prog.sig())?;
    for i in input {
        module.add_port(i.clone());
    }
    let mut decl: Vec<vl::Decl> = Vec::new();
    let dsp_outputs = Output::dsp();
    for i in prog.body() {
        let d: Vec<vl::Decl> = vec_decl_try_from_instr(i)?;
        decl.extend(d);
        if let Some(instr) = i.mach() {
            if instr.op().is_dsp() {
                let term = instr.dst().get_term(0)?;
                let name = tmp_name_try_from_term(term)?;
                if let Some(width) = dsp_outputs.get_width("P") {
                    decl.push(vl::Decl::new_wire(&name, u64::from(*width)));
                }
            }
        }
    }
    let output: Vec<vl::Decl> = wire_try_from_expr(prog.sig().output())?;
    let output_set: HashSet<vl::Decl> = output.into_iter().collect();
    for o in output_set.iter() {
        module.add_port(vl::Port::Output(o.clone()));
    }
    let gnd = Gnd::default();
    let vcc = Vcc::default();
    module.add_decl(gnd.to_decl());
    module.add_decl(vcc.to_decl());
    // only add declarations that are not output
    for d in decl.iter() {
        if !output_set.contains(d) {
            module.add_decl(d.clone());
        }
    }
    module.add_stmt(gnd.to_stmt());
    module.add_stmt(vcc.to_stmt());
    for i in prog.body() {
        let stmt: Vec<vl::Stmt> = stmt_from_instr(i)?;
        for s in stmt {
            module.add_stmt(s);
        }
    }
    Ok(module)
}
