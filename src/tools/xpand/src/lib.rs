pub mod dsp;
pub mod errors;

use crate::errors::Error;
use verilog::ast as vl;
use xir::ast as xir;

pub const CLOCK: &str = "clock";
pub const RESET: &str = "reset";
pub const VCC: &str = "vcc";
pub const GND: &str = "gnd";

fn vec_expr_try_from_term(term: &xir::ExprTerm) -> Result<Vec<vl::Expr>, Error> {
    match term {
        xir::ExprTerm::Any => Ok(vec![vl::Expr::new_ref("")]),
        xir::ExprTerm::Var(id, ty) => {
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
        _ => Err(Error::new_xpand_error("not implemented yet")),
    }
}

fn wire_try_from_term(term: &xir::ExprTerm) -> Result<Vec<vl::Decl>, Error> {
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

fn wire_try_from_tup(tup: &xir::ExprTup) -> Result<Vec<vl::Decl>, Error> {
    let mut decls: Vec<vl::Decl> = Vec::new();
    for term in tup.term() {
        let t: Vec<vl::Decl> = wire_try_from_term(term)?;
        decls.extend(t);
    }
    Ok(decls)
}

fn wire_try_from_expr(expr: &xir::Expr) -> Result<Vec<vl::Decl>, Error> {
    match expr {
        xir::Expr::Tup(tup) => Ok(wire_try_from_tup(tup)?),
        xir::Expr::Term(term) => Ok(wire_try_from_term(term)?),
    }
}

fn input_try_from_sig(sig: &xir::Sig) -> Result<Vec<vl::Port>, Error> {
    let mut port: Vec<vl::Port> = Vec::new();
    port.push(vl::Port::Input(vl::Decl::new_wire(CLOCK, 1)));
    port.push(vl::Port::Input(vl::Decl::new_wire(RESET, 1)));
    let input: Vec<vl::Decl> = wire_try_from_expr(sig.input())?;
    for decl in input {
        port.push(vl::Port::Input(decl.clone()));
    }
    Ok(port)
}

pub fn struct_try_from_xir_prog(prog: &xir::Prog) -> Result<vl::Module, Error> {
    let id = prog.sig().id();
    let mut module = vl::Module::new(&id);
    let input = input_try_from_sig(prog.sig())?;
    for i in input {
        module.add_port(i.clone());
    }
    Ok(module)
}
