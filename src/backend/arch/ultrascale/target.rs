use crate::backend::arch::ultrascale::Ultrascale;
use crate::backend::target::{Descriptor, Target};
use crate::util::file::read_to_string;
use std::str::FromStr;

impl Target for Ultrascale {
    fn to_descriptor(&self) -> Descriptor {
        let spec = read_to_string("src/backend/arch/ultrascale/spec.json");
        Descriptor::from_str(&spec).unwrap()
    }
}
