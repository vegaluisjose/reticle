use crate::errors::Error;
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

    // Output file
    #[structopt(short = "o", long = "output", parse(from_os_str))]
    pub output: Option<PathBuf>,

    // From language
    #[structopt(long = "from", default_value = "ir")]
    pub from: Lang,

    // To language
    #[structopt(long = "to", default_value = "asm")]
    pub to: Lang,
}

impl Opt {
    pub fn input(&self) -> &Path {
        &self.input
    }
    pub fn output(&self) -> Option<&PathBuf> {
        self.output.as_ref()
    }
    pub fn from(&self) -> &Lang {
        &self.from
    }
    pub fn to(&self) -> &Lang {
        &self.to
    }
}

#[derive(Clone, Debug)]
pub enum Lang {
    IR,
    Asm,
    Xir,
    Behav,
    Struct,
}

impl fmt::Display for Lang {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let backend = match self {
            Lang::IR => "ir",
            Lang::Asm => "asm",
            Lang::Xir => "xir",
            Lang::Behav => "behav",
            Lang::Struct => "struct",
        };
        write!(f, "{}", backend)
    }
}

impl FromStr for Lang {
    type Err = Error;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "ir" => Ok(Lang::IR),
            "asm" => Ok(Lang::Asm),
            "xir" => Ok(Lang::Xir),
            "behav" => Ok(Lang::Behav),
            "struct" => Ok(Lang::Struct),
            _ => Err(Error::new_opt_error("Unsupported language")),
        }
    }
}
