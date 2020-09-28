use crate::backend::verilog::Module;
// use crate::lang::parser::parse_from_file;
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

    pub fn is_asm_backend(&self) -> bool {
        match self.backend() {
            Backend::Asm => true,
            _ => false,
        }
    }

    pub fn is_verilog_backend(&self) -> bool {
        match self.backend() {
            Backend::Verilog => true,
            _ => false,
        }
    }
}

#[derive(Clone, Debug)]
pub enum Backend {
    Asm,
    Verilog,
}

// TODO: change this to asm as default
impl Default for Backend {
    fn default() -> Backend {
        Backend::Verilog
    }
}

impl fmt::Display for Backend {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let backend = match self {
            Backend::Asm => "asm",
            Backend::Verilog => "verilog",
        };
        write!(f, "{}", backend)
    }
}

impl FromStr for Backend {
    type Err = String;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "asm" => Ok(Backend::Asm),
            "verilog" => Ok(Backend::Verilog),
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
        // let prog = parse_from_file(self.opts().input());
        if self.opts().is_verilog_backend() {
            let verilog = Module::new("foo");
            self.write_output(&verilog.to_string());
        } else {
            unimplemented!();
        }
    }
}
