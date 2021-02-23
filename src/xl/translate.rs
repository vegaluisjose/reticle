use crate::asm::ast as asm;
use crate::util::errors::Error;
use crate::xl::ast as xl;

pub fn expand_const(instr: &asm::InstrWire) -> Result<Vec<xl::Instr>, Error> {
    let mut body: Vec<xl::Instr> = Vec::new();
    let attr_term = instr.attr().get_term(0)?;
    let value = attr_term.get_val()?;
    let dst_term = instr.dst().get_term(0)?;
    if let Some(width) = dst_term.width() {
        for i in 0..width {
            let lsb = value >> i;
            let mask = lsb & 1;
            let op = if mask == 1 {
                xl::OpBasc::Vcc
            } else {
                xl::OpBasc::Gnd
            };
            let instr = xl::InstrBasc {
                op,
                attr: xl::Expr::default(),
                dst: xl::Expr::default(),
                arg: xl::Expr::default(),
            };
            body.push(xl::Instr::from(instr));
        }
    }
    Ok(body)
}

pub fn test(prog: &asm::Prog) -> Result<xl::Prog, Error> {
    let mut body: Vec<xl::Instr> = Vec::new();
    for instr in prog.body() {
        match instr {
            asm::Instr::Wire(instr) if instr.op() == &asm::OpWire::Con => {
                body.extend(expand_const(instr)?)
            }
            _ => (),
        }
    }
    let mut prog = xl::Prog::default();
    prog.set_body(body);
    Ok(prog)
}
