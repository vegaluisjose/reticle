pub mod errors;

use crate::errors::Error;
use ir::ast as ir;
use verilog::ast as vl;

pub fn behav_try_from_ir_def(def: &ir::Def) -> Result<vl::Module, Error> {
    let id = def.sig().id();
    let module = vl::Module::new(&id);
    Ok(module)
}

pub fn behav_try_from_ir_prog(prog: &ir::Prog) -> Result<vl::Module, Error> {
    if let Some(def) = prog.get("main") {
        Ok(behav_try_from_ir_def(def)?)
    } else {
        Err(Error::new_bline_error("main not found"))
    }
}
