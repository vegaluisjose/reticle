use crate::backend::arch::ultrascale::Ultrascale;
use crate::backend::target::{Descriptor, Target};
use std::str::FromStr;

impl Target for Ultrascale {
    fn to_descriptor(&self) -> Descriptor {
        Descriptor::from_str(&self.spec()).unwrap()
    }
}
