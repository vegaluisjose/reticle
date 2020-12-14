use crate::asm::ast as asm;
use crate::asm::parser::parse_from_file as parse_asm_from_file;
use crate::backend::verilog::{Attribute, Module};
use crate::lang::parser::parse_from_file;
use crate::util::file::write_to_file;
use std::fmt;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use structopt::StructOpt;

#[derive(Clone, Debug, StructOpt)]
#[structopt(
    name = env!("CARGO_PKG_NAME"),
    version = env!("CARGO_PKG_VERSION"),
    author = env!("CARGO_PKG_AUTHORS")
)]
pub struct Opt {
    // Input file
    #[structopt(parse(from_os_str))]
    pub input: PathBuf,

    // Backend
    #[structopt(short = "b", long = "backend", default_value)]
    pub backend: Backend,

    // Input file is assembly
    #[structopt(long)]
    pub asm: bool,

    // Add dsp compiler hint for verilog backend
    #[structopt(long)]
    pub use_dsp: bool,

    // Output file
    #[structopt(short = "o", long = "output", parse(from_os_str))]
    pub output: Option<PathBuf>,
}

impl Opt {
    pub fn input(&self) -> &Path {
        &self.input
    }

    pub fn output(&self) -> Option<&PathBuf> {
        self.output.as_ref()
    }

    pub fn backend(&self) -> &Backend {
        &self.backend
    }

    pub fn asm(&self) -> bool {
        self.asm
    }

    pub fn use_dsp(&self) -> bool {
        self.use_dsp
    }
}

#[derive(Clone, Debug)]
pub enum Backend {
    Asm,
    Behavioral,
    Structural,
}

impl Default for Backend {
    fn default() -> Backend {
        Backend::Structural
    }
}

impl fmt::Display for Backend {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let backend = match self {
            Backend::Asm => "asm",
            Backend::Behavioral => "behavioral",
            Backend::Structural => "structural",
        };
        write!(f, "{}", backend)
    }
}

impl FromStr for Backend {
    type Err = String;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "asm" => Ok(Backend::Asm),
            "behavioral" => Ok(Backend::Behavioral),
            "structural" => Ok(Backend::Structural),
            _ => Err(format!("Error: {} is not valid backend", input)),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Translate {
    pub opts: Opt,
}

impl Default for Translate {
    fn default() -> Translate {
        Translate {
            opts: Opt::from_args(),
        }
    }
}

impl Translate {
    pub fn new(opts: Opt) -> Translate {
        Translate { opts }
    }

    pub fn opts(&self) -> &Opt {
        &self.opts
    }

    fn write_output(&self, contents: &str) {
        if let Some(output) = self.opts().output() {
            write_to_file(output, contents);
        } else {
            println!("{}", contents);
        }
    }

    pub fn run(&self) {
        match self.opts().backend() {
            Backend::Behavioral => {
                let prog = parse_from_file(self.opts().input());
                let mut m = Module::from(prog);
                if self.opts().use_dsp() {
                    let mut attr = Attribute::default();
                    attr.add_stmt("use_dsp", "yes");
                    m.set_attr(attr);
                }
                self.write_output(&m.to_string());
            }
            Backend::Structural => {
                let prog_asm = if self.opts().asm() {
                    parse_asm_from_file(self.opts().input())
                } else {
                    let prog = parse_from_file(self.opts().input());
                    asm::Prog::from(prog)
                };
                let m = Module::from(prog_asm);
                self.write_output(&m.to_string());
            }
            Backend::Asm => {
                let prog = parse_from_file(self.opts().input());
                let asm = asm::Prog::from(prog);
                self.write_output(&asm.to_string());
            }
        }
    }
}