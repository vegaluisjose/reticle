use std::fmt;
use std::path::PathBuf;
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
}

#[derive(Clone, Debug)]
pub enum Backend {
    Loc,
    Asm,
    Verilog,
}

impl Default for Backend {
    fn default() -> Backend {
        Backend::Verilog
    }
}

impl fmt::Display for Backend {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let backend = match self {
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
            "loc" => Ok(Backend::Loc),
            "asm" => Ok(Backend::Asm),
            "verilog" => Ok(Backend::Verilog),
            _ => Err(format!("Error: {} is not valid backend", input)),
        }
    }
}
