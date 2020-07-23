use crate::backend::target::descriptor::Descriptor;
use crate::backend::target::Target;
use crate::util::file::read_to_string;
use std::str::FromStr;

pub struct Ultrascale {
    pub spec: String,
}

impl Target for Ultrascale {
    fn to_descriptor(&self) -> Descriptor {
        Descriptor::from_str(&self.spec).unwrap()
    }
}

impl Default for Ultrascale {
    fn default() -> Self {
        Ultrascale {
            spec: read_to_string("isa/ultrascale.json"),
        }
    }
}
