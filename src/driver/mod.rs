pub mod opt;

use crate::driver::opt::Opt;
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
}
