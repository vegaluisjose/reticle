use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Mem {
    offset: u32,
    values: Vec<u8>,
}

pub type Mmap = HashMap<String, Mem>;

impl Mem {
    #[must_use]
    pub fn new(offset: u32) -> Self {
        Mem {
            offset,
            values: Vec::new(),
        }
    }
    #[must_use]
    pub fn offset(&self) -> u32 {
        self.offset
    }
    #[must_use]
    pub fn values(&self) -> &Vec<u8> {
        &self.values
    }
    pub fn add_value(&mut self, value: u8) {
        self.values.push(value);
    }
}
