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
pub struct TranslateOption {
    // Input file
    #[structopt(parse(from_os_str))]
    pub input: PathBuf,

    // Output file
    #[structopt(short = "o", long = "output", parse(from_os_str))]
    pub output: Option<PathBuf>,

    // Translation type
    #[structopt(long = "type", default_value)]
    pub ty: TranslationTy,
}

impl TranslateOption {
    pub fn input(&self) -> &Path {
        &self.input
    }
    pub fn output(&self) -> Option<&PathBuf> {
        self.output.as_ref()
    }
    pub fn ty(&self) -> &TranslationTy {
        &self.ty
    }
}

#[derive(Clone, Debug)]
pub enum TranslationTy {
    IrToAsm,
}

impl Default for TranslationTy {
    fn default() -> Self {
        TranslationTy::IrToAsm
    }
}

impl fmt::Display for TranslationTy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let backend = match self {
            TranslationTy::IrToAsm => "ir-to-asm",
        };
        write!(f, "{}", backend)
    }
}

impl FromStr for TranslationTy {
    type Err = String;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "ir-to-asm" => Ok(TranslationTy::IrToAsm),
            _ => Err(format!("Error: {} invalid option", input)),
        }
    }
}

#[derive(Clone, Debug)]
pub struct TranslateDriver {
    pub opts: TranslateOption,
}

impl Default for TranslateDriver {
    fn default() -> Self {
        TranslateDriver {
            opts: TranslateOption::from_args(),
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

impl TranslateDriver {
    pub fn new(opts: TranslateOption) -> TranslateDriver {
        TranslateDriver { opts }
    }
    pub fn opts(&self) -> &TranslateOption {
        &self.opts
    }
    pub fn run(&self) {
        let output = self.opts().output();
        match self.opts().ty() {
            TranslationTy::IrToAsm => write_output(output, "here"),
        }
    }
}
