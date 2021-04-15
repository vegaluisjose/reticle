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

    // Language
    #[structopt(long = "lang", default_value = "asm")]
    pub lang: Lang,

    // Optimization
    #[structopt(long = "opt", default_value = "cascade")]
    pub optimization: Optimization,
}

impl Opt {
    pub fn input(&self) -> &Path {
        &self.input
    }
    pub fn output(&self) -> Option<&PathBuf> {
        self.output.as_ref()
    }
    pub fn lang(&self) -> &Lang {
        &self.lang
    }
    pub fn optimization(&self) -> &Optimization {
        &self.optimization
    }
}

#[derive(Clone, Debug)]
pub enum Lang {
    Asm,
}

#[derive(Clone, Debug)]
pub enum Optimization {
    Cascade,
}

impl fmt::Display for Lang {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let lang = match self {
            Lang::Asm => "asm",
        };
        write!(f, "{}", lang)
    }
}

impl FromStr for Lang {
    type Err = Error;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "asm" => Ok(Lang::Asm),
            _ => Err(Error::new_opt_error("Unsupported language")),
        }
    }
}

impl fmt::Display for Optimization {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let opt = match self {
            Optimization::Cascade => "cascade",
        };
        write!(f, "{}", opt)
    }
}

impl FromStr for Optimization {
    type Err = Error;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "cascade" => Ok(Optimization::Cascade),
            _ => Err(Error::new_opt_error("Unsupported language")),
        }
    }
}
