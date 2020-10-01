use crate::backend::arch::ultrascale::Ultrascale;
use crate::backend::target::{Descriptor, Target};
use crate::util::file::{create_absolute, read_to_string};
use std::str::FromStr;

impl Target for Ultrascale {
    fn to_descriptor(&self) -> Descriptor {
        let spec_path = create_absolute("src/backend/arch/ultrascale/spec.json");
        let spec = read_to_string(&spec_path);
        Descriptor::from_str(&spec).unwrap()
    }
}
