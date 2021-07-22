use crate::errors::Error;
use crate::opt::{Lang, Opt};
use asm::parser::Parser as AsmParser;
use bler::try_from_asm_prog as asm_try_into_xir;
use bline::try_from_ir_prog as ir_try_into_behav;
use io::write_to_file;
use ir::parser::Parser as IrParser;
use isel::try_from_ir_prog as ir_try_into_asm;
use std::path::PathBuf;
use structopt::StructOpt;
use xir::parser::Parser as XirParser;
use xpand::try_from_xir_prog as xir_try_into_struct;

#[derive(Clone, Debug)]
pub struct Driver {
    pub opts: Opt,
}

impl Default for Driver {
    fn default() -> Self {
        Driver {
            opts: Opt::from_args(),
        }
    }
}

fn write_output(path: Option<&PathBuf>, contents: &str) {
    if let Some(output) = path {
        write_to_file(output, contents);
    } else {
        println!("{}", contents);
    }
}

impl Driver {
    pub fn new(opts: Opt) -> Driver {
        Driver { opts }
    }
    pub fn opts(&self) -> &Opt {
        &self.opts
    }
    pub fn run(&self) -> Result<(), Error> {
        let input = self.opts().input();
        let output = self.opts().output();
        match (self.opts().from(), self.opts().to(), self.opts().mmap()) {
            (Lang::Ir, Lang::Asm, _) => {
                let ir = IrParser::parse_from_file(input)?;
                let asm = ir_try_into_asm(&ir)?;
                write_output(output, &asm.to_string());
                Ok(())
            }
            (Lang::Ir, Lang::Xir, _) => {
                let ir = IrParser::parse_from_file(input)?;
                let asm = ir_try_into_asm(&ir)?;
                let xir = asm_try_into_xir(&asm)?;
                write_output(output, &xir.to_string());
                Ok(())
            }
            (Lang::Ir, Lang::Behav, _) => {
                let ir = IrParser::parse_from_file(input)?;
                let behav_prog = ir_try_into_behav(&ir)?;
                write_output(output, &behav_prog.to_string());
                Ok(())
            }
            (Lang::Asm, Lang::Xir, _) => {
                let prog = AsmParser::parse_from_file(input)?;
                let xir = asm_try_into_xir(&prog)?;
                write_output(output, &xir.to_string());
                Ok(())
            }
            (Lang::Asm, Lang::Struct, _) => {
                let prog = AsmParser::parse_from_file(input)?;
                let xir = asm_try_into_xir(&prog)?;
                let sct = xir_try_into_struct(&xir, None)?;
                write_output(output, &sct.to_string());
                Ok(())
            }
            (Lang::Ir, Lang::Struct, None) => {
                let ir = IrParser::parse_from_file(input)?;
                let asm = ir_try_into_asm(&ir)?;
                let xir = asm_try_into_xir(&asm)?;
                let sct = xir_try_into_struct(&xir, None)?;
                write_output(output, &sct.to_string());
                Ok(())
            }
            (Lang::Xir, Lang::Struct, None) => {
                let prog = XirParser::parse_from_file(input)?;
                let sct = xir_try_into_struct(&prog, None)?;
                write_output(output, &sct.to_string());
                Ok(())
            }
            (Lang::Ir, Lang::Struct, Some(path)) => {
                let ir = IrParser::parse_from_file(input)?;
                let asm = ir_try_into_asm(&ir)?;
                let xir = asm_try_into_xir(&asm)?;
                let mmap = Some(mmap::Mmap::from_file(path));
                let sct = xir_try_into_struct(&xir, mmap.as_ref())?;
                write_output(output, &sct.to_string());
                Ok(())
            }
            (Lang::Xir, Lang::Struct, Some(path)) => {
                let prog = XirParser::parse_from_file(input)?;
                let mmap = Some(mmap::Mmap::from_file(path));
                let sct = xir_try_into_struct(&prog, mmap.as_ref())?;
                write_output(output, &sct.to_string());
                Ok(())
            }
            (_, _, _) => Err(Error::new_driver_error("Unsupported conversion")),
        }
    }
}
