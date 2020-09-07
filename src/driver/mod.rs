pub mod opt;

use crate::backend::verilog::Module;
use crate::driver::opt::Opt;
use crate::frontend::parser::parse_from_file;
use crate::passes::map::{asmgen, locgen};
use crate::util::file::write_to_file;
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

    fn write_output(&self, contents: &str) {
        if let Some(output) = self.opts().output() {
            write_to_file(output, contents);
        } else {
            println!("{}", contents);
        }
    }

    pub fn run(&self) {
        let prog = parse_from_file(self.opts().input());
        if self.opts().is_reticle_backend() {
            self.write_output(&prog.to_string());
        } else if self.opts().is_loc_backend() {
            let prog_with_loc = locgen(prog);
            self.write_output(&prog_with_loc.to_string());
        } else if self.opts().is_asm_backend() {
            let asm = asmgen(prog, self.opts().check());
            self.write_output(&asm.to_string());
        } else if self.opts().is_verilog_backend() {
            let asm = asmgen(prog, self.opts().check());
            let verilog = Module::from(asm);
            self.write_output(&verilog.to_string());
        }
    }
}