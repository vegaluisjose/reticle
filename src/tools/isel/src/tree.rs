use crate::errors::Error;
use asm::ast as asm;
use ir::ast as ir;

pub fn select(_: &ir::Prog) -> Result<asm::Prog, Error> {
    let prog = asm::Prog::default();
    Ok(prog)
}
