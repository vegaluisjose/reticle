pub mod opt;

use crate::backend::verilog::Module;
use crate::driver::opt::Opt;
use crate::frontend::parser::parse_from_file;
use crate::passes::map::{asmgen, locgen};
use structopt::StructOpt;

#[derive(Clone, Debug)]
pub struct Driver {
    pub opts: Opt,
}

impl Default for Driver {
    fn default() -> Driver {
        Driver {
            opts: Opt::from_args(),
        }
    }
}

impl Driver {
    pub fn new(opts: Opt) -> Driver {
        Driver { opts }
    }

    pub fn opts(&self) -> &Opt {
        &self.opts
    }

    pub fn run(&self) {
        let prog = parse_from_file(self.opts().input());
        if self.opts().is_reticle_backend() {
            println!("{}", prog);
        } else if self.opts().is_loc_backend() {
            let prog_with_loc = locgen(prog);
            println!("{}", prog_with_loc);
        } else if self.opts().is_asm_backend() {
            let asm = asmgen(prog, true);
            println!("{}", asm);
        } else if self.opts().is_verilog_backend() {
            let asm = asmgen(prog, true);
            let verilog = Module::from(asm);
            println!("{}", verilog);
        }
    }
}
