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

    // Disable check
    #[structopt(long)]
    pub no_check: bool,

    // Backend
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

    pub fn check(&self) -> bool {
        !self.no_check
    }

    pub fn backend(&self) -> &Backend {
        &self.backend
    }

    pub fn is_reticle_backend(&self) -> bool {
        match self.backend() {
            Backend::Reticle => true,
            _ => false,
        }
    }

    pub fn is_loc_backend(&self) -> bool {
        match self.backend() {
            Backend::Loc => true,
            _ => false,
        }
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
    Reticle,
    Loc,
    Asm,
    Verilog,
}

impl Default for Backend {
    fn default() -> Backend {
        Backend::Reticle
    }
}

impl fmt::Display for Backend {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let backend = match self {
            Backend::Reticle => "reticle",
            Backend::Loc => "loc",
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
            "reticle" => Ok(Backend::Reticle),
            "loc" => Ok(Backend::Loc),
            "asm" => Ok(Backend::Asm),
            "verilog" => Ok(Backend::Verilog),
            _ => Err(format!("Error: {} is not valid backend", input)),
        }
    }
}
