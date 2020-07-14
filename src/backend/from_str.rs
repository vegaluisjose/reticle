use crate::backend::descriptor::Target;
use serde_json;
use std::str::FromStr;

impl FromStr for Target {
    type Err = ();
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(Target::from_serial(
            serde_json::from_str(input).expect("Error: parsing json"),
        ))
    }
}
