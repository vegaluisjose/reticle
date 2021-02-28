use crate::asm::ast as asm;
use crate::assembler::Assembler;
use crate::tdl::ast as tdl;
use crate::tdl::parser::TDLParser;
use crate::util::errors::Error;
use crate::util::file::create_abs_path;
use crate::xl::ast as xl;
use std::collections::HashMap;
use std::convert::TryFrom;

fn get_imp() -> Result<HashMap<String, tdl::Imp>, Error> {
    let dsp_tdl = create_abs_path("examples/target/ultrascale/dsp.tdl");
    let lut_tdl = create_abs_path("examples/target/ultrascale/lut.tdl");
    let lut = TDLParser::parse_from_file(lut_tdl)?;
    let dsp = TDLParser::parse_from_file(dsp_tdl)?;
    let mut imp = lut.imp().clone();
    imp.extend(dsp.imp().clone());
    Ok(imp)
}

impl TryFrom<asm::Prog> for Assembler {
    type Error = Error;
    fn try_from(input: asm::Prog) -> Result<Self, Self::Error> {
        let mut assembler = Assembler::new(input.sig());
        assembler.set_imp(get_imp()?);
        for instr in input.body() {
            match instr {
                asm::Instr::Wire(instr) if instr.op() == &asm::OpWire::Con => {
                    assembler.expand_instr_const(instr)?;
                }
                asm::Instr::Wire(instr) if instr.op() == &asm::OpWire::Id => {
                    assembler.expand_instr_id(instr)?;
                }
                asm::Instr::Asm(instr) => assembler.expand_instr_asm(instr)?,
                _ => (),
            }
        }
        Ok(assembler)
    }
}

impl TryFrom<asm::Prog> for xl::Prog {
    type Error = Error;
    fn try_from(input: asm::Prog) -> Result<Self, Self::Error> {
        let assembler = Assembler::try_from(input)?;
        let mut prog = xl::Prog::default();
        prog.set_sig(assembler.sig().clone());
        prog.set_body(assembler.body().clone());
        Ok(prog)
    }
}
