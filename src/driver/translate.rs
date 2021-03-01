use crate::asm::ast as asm;
use crate::asm::parser::AsmParser;
use crate::ir::parser::IRParser;
use crate::util::errors::Error;
use crate::util::file::write_to_file;
use crate::verilog::ast as vl;
use crate::xl::ast as xl;
use crate::xl::parser::XLParser;
use std::convert::TryFrom;
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

    // FromTo
    #[structopt(long = "fromto", default_value)]
    pub fromto: FromTo,
}

impl TranslateOption {
    pub fn input(&self) -> &Path {
        &self.input
    }
    pub fn output(&self) -> Option<&PathBuf> {
        self.output.as_ref()
    }
    pub fn fromto(&self) -> &FromTo {
        &self.fromto
    }
}

#[derive(Clone, Debug)]
pub enum FromTo {
    IRToAsm,
    IRToBehav,
    AsmToXL,
    XLToVerilog,
}

impl Default for FromTo {
    fn default() -> Self {
        FromTo::IRToAsm
    }
}

impl fmt::Display for FromTo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let backend = match self {
            FromTo::IRToAsm => "ir-to-asm",
            FromTo::IRToBehav => "ir-to-behav",
            FromTo::AsmToXL => "asm-to-xl",
            FromTo::XLToVerilog => "xl-to-verilog",
        };
        write!(f, "{}", backend)
    }
}

impl FromStr for FromTo {
    type Err = Error;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "ir-to-asm" => Ok(FromTo::IRToAsm),
            "ir-to-behav" => Ok(FromTo::IRToBehav),
            "asm-to-xl" => Ok(FromTo::AsmToXL),
            "xl-to-verilog" => Ok(FromTo::XLToVerilog),
            _ => Err(Error::new_opt_error("invalid options")),
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
    pub fn run(&self) -> Result<(), Error> {
        let input = self.opts().input();
        let output = self.opts().output();
        match self.opts().fromto() {
            FromTo::IRToAsm => {
                let prog = IRParser::parse_from_file(input)?;
                let asm = asm::Prog::try_from(prog)?;
                write_output(output, &asm.to_string());
            }
            FromTo::IRToBehav => {
                let prog = IRParser::parse_from_file(input)?;
                let vl = vl::Module::try_from(prog)?;
                write_output(output, &vl.to_string());
            }
            FromTo::AsmToXL => {
                let prog = AsmParser::parse_from_file(input)?;
                let xl = xl::Prog::try_from(prog)?;
                write_output(output, &xl.to_string());
            }
            FromTo::XLToVerilog => {
                let prog = XLParser::parse_from_file(input)?;
                let vl = vl::Module::try_from(prog)?;
                write_output(output, &vl.to_string());
            }
        }
        Ok(())
    }
}
