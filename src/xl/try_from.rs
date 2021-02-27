use crate::asm::ast as asm;
use crate::util::errors::Error;
use crate::xl::ast::*;
use crate::xl::expander::Expander;
use std::convert::TryFrom;

impl TryFrom<OptVal> for u64 {
    type Error = Error;
    fn try_from(val: OptVal) -> Result<Self, Self::Error> {
        match val {
            OptVal::UInt(n) => Ok(n),
            _ => Err(Error::new_conv_error("not a uint value")),
        }
    }
}

impl TryFrom<OptVal> for i32 {
    type Error = Error;
    fn try_from(val: OptVal) -> Result<Self, Self::Error> {
        match val {
            OptVal::UInt(a) => {
                if let Ok(b) = i32::try_from(a) {
                    Ok(b)
                } else {
                    Err(Error::new_conv_error("converting u64 to i32"))
                }
            }
            _ => Err(Error::new_conv_error("not a uint value")),
        }
    }
}

impl TryFrom<asm::Prog> for Expander {
    type Error = Error;
    fn try_from(input: asm::Prog) -> Result<Self, Self::Error> {
        let mut expander = Expander::default();
        expander.set_sig(input.sig().clone());
        for instr in input.body() {
            match instr {
                asm::Instr::Wire(instr) if instr.op() == &asm::OpWire::Con => {
                    expander.expand_const(instr)?;
                }
                asm::Instr::Asm(instr) => {
                    let new_instr = expander.rename_instr_asm(instr)?;
                    println!("old:{}\nnew:{}", instr, new_instr);
                }
                _ => (),
            }
        }
        Ok(expander)
    }
}

impl TryFrom<asm::Prog> for Prog {
    type Error = Error;
    fn try_from(input: asm::Prog) -> Result<Self, Self::Error> {
        let expander = Expander::try_from(input)?;
        let mut prog = Prog::default();
        prog.set_sig(expander.sig().clone());
        prog.set_body(expander.body().clone());
        Ok(prog)
    }
}
