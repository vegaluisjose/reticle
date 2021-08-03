pub mod bram;
pub mod carry;
pub mod cat;
pub mod decl;
pub mod display;
pub mod dsp;
pub mod errors;
pub mod expr;
pub mod ext;
pub mod fdre;
pub mod fdse;
pub mod gnd;
pub mod id;
pub mod instance;
pub mod loc;
pub mod lram;
pub mod lut;
pub mod param;
pub mod port;
pub mod to_verilog;
pub mod vcc;

use crate::decl::ToDecl;
use crate::errors::Error;
use crate::instance::ToInstance;
use crate::port::DefaultPort;
use crate::to_verilog::{ToVerilogDecl, ToVerilogInstance};
use crate::vcc::Vcc;
use bline::{
    input_try_from_sig, vec_expr_try_from_expr, vec_expr_try_from_term, wire_try_from_expr,
};
use prim::ultrascale::clock::CLOCK;
use prim::ultrascale::gnd::Gnd;
use prim::ultrascale::reset::RESET;
use std::collections::HashSet;
use std::convert::TryInto;
use verilog::ast as vl;
use xir::ast as xir;

pub fn create_literal(width: u64, value: i64) -> vl::Expr {
    assert!(width != 0);
    use prim::ultrascale::gnd::GND;
    if width == 1 {
        let mask = value & 1;
        let is_one = mask == 1;
        if is_one {
            vl::Expr::new_ref(vcc::VCC)
        } else {
            vl::Expr::new_ref(GND)
        }
    } else if width <= 64 {
        let mut concat = vl::ExprConcat::default();
        for i in 0..width {
            let i = i as i64;
            let shift = value >> i;
            let mask = shift & 1;
            let is_one = mask == 1;
            if is_one {
                concat.add_expr(vl::Expr::new_ref(vcc::VCC));
            } else {
                concat.add_expr(vl::Expr::new_ref(GND));
            }
        }
        vl::Expr::from(concat)
    } else {
        let mut concat = vl::ExprConcat::default();
        for i in 0..64 {
            let i = i as i64;
            let shift = value >> i;
            let mask = shift & 1;
            let is_one = mask == 1;
            if is_one {
                concat.add_expr(vl::Expr::new_ref(vcc::VCC));
            } else {
                concat.add_expr(vl::Expr::new_ref(GND));
            }
        }
        let pad = width - 64;
        for _ in 0..pad {
            concat.add_expr(vl::Expr::new_ref(GND));
        }
        vl::Expr::from(concat)
    }
}

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

fn stmt_from_mach(instr: &xir::InstrMach) -> Result<Vec<vl::Stmt>, Error> {
    match instr.op() {
        xir::OpMach::Lut1 => lut::lut1_from_mach(instr),
        xir::OpMach::Lut2 => lut::lut2_from_mach(instr),
        xir::OpMach::Lut3 => lut::lut3_from_mach(instr),
        xir::OpMach::Lut4 => lut::lut4_from_mach(instr),
        xir::OpMach::Lut5 => lut::lut5_from_mach(instr),
        xir::OpMach::Lut6 => lut::lut6_from_mach(instr),
        xir::OpMach::Fdre => fdre::fdre_from_mach(instr),
        xir::OpMach::CarryAdd => carry::carryadd_from_mach(instr),
        xir::OpMach::VecAddRegA => dsp::vaddrega_from_mach(instr),
        xir::OpMach::MulAddRegA => dsp::muladdrega_from_mach(instr),
        xir::OpMach::Lrom => lram::rom_from_mach(instr),
        xir::OpMach::Brom => bram::rom_from_mach(instr),
        _ => {
            let err = format!("{} unsupported instruction", instr);
            Err(Error::new_xpand_error(&err))
        }
    }
}

fn stmt_from_basc(instr: &xir::InstrBasc) -> Result<Vec<vl::Stmt>, Error> {
    match instr.op() {
        xir::OpBasc::Ext => ext::from_basc(instr),
        xir::OpBasc::Cat => cat::from_basc(instr),
        xir::OpBasc::Gnd => gnd::from_basc(instr),
        xir::OpBasc::Vcc => vcc::from_basc(instr),
        xir::OpBasc::Id => id::from_basc(instr),
    }
}

pub fn try_from_xir_prog(prog: &xir::Prog, mmap: Option<&mmap::Mmap>) -> Result<vl::Module, Error> {
    let id = prog.sig().id();
    let mut module = vl::Module::new(&id);
    let input = input_try_from_sig(prog.sig())?;
    for i in input {
        module.add_port(i.clone());
    }
    let mut decl: Vec<vl::Decl> = Vec::new();
    let dsp_outputs = dsp::Dsp::default_output_port();
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
    for o in output.iter() {
        module.add_port(vl::Port::Output(o.clone()));
    }
    let gnd = Gnd::default();
    let vcc = Vcc::default();
    module.add_decl(gnd.to_decl());
    module.add_decl(vcc.to_decl());
    // only add declarations that are not output
    let output_set: HashSet<vl::Decl> = output.into_iter().collect();
    for d in decl.iter() {
        if !output_set.contains(d) {
            module.add_decl(d.clone());
        }
    }
    module.add_stmt(gnd.to_stmt());
    module.add_stmt(vcc.to_stmt());
    for instr in prog.body() {
        let stmt = match instr {
            xir::Instr::Basc(basc) => stmt_from_basc(basc)?,
            xir::Instr::Mach(mach) => {
                let id = mach.dst().get_id(0)?;
                match mmap {
                    Some(m) => {
                        if let Some(mem) = m.get(&id) {
                            let mut instr_mach = mach.clone();
                            instr_mach.set_mem(mem.clone());
                            stmt_from_mach(&instr_mach)?
                        } else {
                            stmt_from_mach(mach)?
                        }
                    }
                    _ => stmt_from_mach(mach)?,
                }
            }
        };
        for s in stmt {
            module.add_stmt(s);
        }
    }
    Ok(module)
}
