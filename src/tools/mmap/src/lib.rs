use derive_more::{Deref, DerefMut};
use io::read_to_string;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Mem {
    offset: u32,
    values: Vec<u8>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Deref, DerefMut, Serialize, Deserialize)]
pub struct Mmap(HashMap<String, Mem>);

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

impl Mmap {
    #[must_use]
    pub fn new() -> Self {
        Mmap::default()
    }
    #[must_use]
    pub fn from_file<P: AsRef<Path>>(path: P) -> Mmap {
        let content = read_to_string(path);
        serde_json::from_str(&content).unwrap()
    }
}

pub fn mmap_from_file<P: AsRef<Path>>(path: P) -> Mmap {
    let content = read_to_string(path);
    serde_json::from_str(&content).unwrap()
}
