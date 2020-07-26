use crate::backend::target::descriptor::Descriptor;
use crate::backend::target::Target;
use crate::util::file::read_to_string;
use std::str::FromStr;

pub struct Ultrascale {
    pub spec: String,
}

impl Default for Ultrascale {
    fn default() -> Self {
        Ultrascale {
            spec: read_to_string("src/backend/arch/ultrascale.json"),
        }
    }
}

impl Ultrascale {
    fn spec(&self) -> String {
        self.spec.to_string()
    }
}

impl Target for Ultrascale {
    fn to_descriptor(&self) -> Descriptor {
        Descriptor::from_str(&self.spec()).unwrap()
    }
}
