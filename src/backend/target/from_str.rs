use crate::backend::target::descriptor::Descriptor;
use crate::backend::target::spec::Spec;
use std::str::FromStr;

impl FromStr for Descriptor {
    type Err = ();
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let spec: Spec = serde_json::from_str(input).expect("Error: parsing json");
        Ok(Descriptor::from(spec))
    }
}
