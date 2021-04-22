use crate::errors::Error;
use crate::opt::{Lang, Opt};
use bler::try_assemble_from_asm_prog;
use bline::behav_try_from_ir_prog;
use io::file::write_to_file;
use ir::parser::Parser as IRParser;
use isel::try_select_from_ir_prog;
use std::path::PathBuf;
use structopt::StructOpt;

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
        match (self.opts().from(), self.opts().to()) {
            (Lang::IR, Lang::Asm) => {
                let ir_prog = IRParser::parse_from_file(input)?;
                let asm_prog = try_select_from_ir_prog(&ir_prog)?;
                write_output(output, &asm_prog.to_string());
                Ok(())
            }
            (Lang::IR, Lang::Xir) => {
                let ir_prog = IRParser::parse_from_file(input)?;
                let asm_prog = try_select_from_ir_prog(&ir_prog)?;
                let xir_prog = try_assemble_from_asm_prog(asm_prog)?;
                write_output(output, &xir_prog.to_string());
                Ok(())
            }
            (Lang::IR, Lang::Behav) => {
                let ir_prog = IRParser::parse_from_file(input)?;
                let behav_prog = behav_try_from_ir_prog(&ir_prog)?;
                write_output(output, &behav_prog.to_string());
                Ok(())
            }
            (_, _) => Err(Error::new_driver_error("Unsupported conversion")),
        }
    }
}
