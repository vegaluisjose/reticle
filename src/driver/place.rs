use crate::asm::parser::AsmParser;
use crate::placer::{place_dsp_from_prog, place_lut_from_prog};
use crate::util::errors::Error;
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
pub struct PlaceOption {
    // Input file
    #[structopt(parse(from_os_str))]
    pub input: PathBuf,

    // Output file
    #[structopt(short = "o", long = "output", parse(from_os_str))]
    pub output: Option<PathBuf>,

    // What to place
    #[structopt(short = "p", long = "prim", default_value)]
    pub ty: PlaceTy,
}

impl PlaceOption {
    pub fn input(&self) -> &Path {
        &self.input
    }
    pub fn output(&self) -> Option<&PathBuf> {
        self.output.as_ref()
    }
    pub fn ty(&self) -> &PlaceTy {
        &self.ty
    }
}

#[derive(Clone, Debug)]
pub enum PlaceTy {
    All,
    Dsp,
    Lut,
}

impl Default for PlaceTy {
    fn default() -> Self {
        PlaceTy::All
    }
}

impl fmt::Display for PlaceTy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ty = match self {
            PlaceTy::All => "all",
            PlaceTy::Dsp => "dsp",
            PlaceTy::Lut => "lut",
        };
        write!(f, "{}", ty)
    }
}

impl FromStr for PlaceTy {
    type Err = Error;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "all" => Ok(PlaceTy::All),
            "dsp" => Ok(PlaceTy::Dsp),
            "lut" => Ok(PlaceTy::Lut),
            _ => Err(Error::new_opt_error("invalid options")),
        }
    }
}

#[derive(Clone, Debug)]
pub struct PlaceDriver {
    pub opts: PlaceOption,
}

impl Default for PlaceDriver {
    fn default() -> Self {
        PlaceDriver {
            opts: PlaceOption::from_args(),
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

impl PlaceDriver {
    pub fn new(opts: PlaceOption) -> PlaceDriver {
        PlaceDriver { opts }
    }
    pub fn opts(&self) -> &PlaceOption {
        &self.opts
    }
    pub fn run(&self) -> Result<(), Error> {
        let input = self.opts().input();
        let input = AsmParser::parse_from_file(input)?;
        let output = self.opts().output();
        let asm = match self.opts().ty() {
            PlaceTy::All => {
                let asm = place_lut_from_prog(&input)?;
                place_dsp_from_prog(&asm)?
            }
            PlaceTy::Lut => place_lut_from_prog(&input)?,
            PlaceTy::Dsp => place_dsp_from_prog(&input)?,
        };
        write_output(output, &asm.to_string());
        Ok(())
    }
}
