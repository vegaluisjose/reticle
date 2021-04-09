use crate::tree::*;
use std::collections::HashMap;

impl Default for Tree {
    fn default() -> Self {
        Tree {
            index: 0,
            node: HashMap::new(),
            edge: HashMap::new(),
        }
    }
}
