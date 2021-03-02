use crate::asm::parser::AsmParser;
use crate::optimizer::cascader::cascade;
use crate::util::errors::Error;
use crate::util::file::write_to_file;
use std::path::{Path, PathBuf};
use structopt::StructOpt;

#[derive(Clone, Debug, StructOpt)]
#[structopt(
    name = env!("CARGO_PKG_NAME"),
    version = env!("CARGO_PKG_VERSION"),
    author = env!("CARGO_PKG_AUTHORS")
)]
pub struct OptimizeOption {
    // Input file
    #[structopt(parse(from_os_str))]
    pub input: PathBuf,

    // Output file
    #[structopt(short = "o", long = "output", parse(from_os_str))]
    pub output: Option<PathBuf>,
}

impl OptimizeOption {
    pub fn input(&self) -> &Path {
        &self.input
    }
    pub fn output(&self) -> Option<&PathBuf> {
        self.output.as_ref()
    }
}

#[derive(Clone, Debug)]
pub struct OptimizeDriver {
    pub opts: OptimizeOption,
}

impl Default for OptimizeDriver {
    fn default() -> Self {
        OptimizeDriver {
            opts: OptimizeOption::from_args(),
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

impl OptimizeDriver {
    pub fn new(opts: OptimizeOption) -> OptimizeDriver {
        OptimizeDriver { opts }
    }
    pub fn opts(&self) -> &OptimizeOption {
        &self.opts
    }
    pub fn run(&self) -> Result<(), Error> {
        let input = self.opts().input();
        let output = self.opts().output();
        let prog = AsmParser::parse_from_file(input)?;
        let prog = cascade(&prog)?;
        write_output(output, &prog.to_string());
        Ok(())
    }
}
