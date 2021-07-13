use crate::cascader::cascader;
use crate::errors::Error;
use crate::opt::{Opt, Optimization};
use asm::parser::Parser as AsmParser;
use io::write_to_file;
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
        match self.opts().optimization() {
            Optimization::Cascade => {
                let prog = AsmParser::parse_from_file(input)?;
                let opt = cascader(&prog)?;
                write_output(output, &opt.to_string());
                Ok(())
            }
        }
    }
}
