pub mod algo;
pub mod display;
pub mod from;
pub mod from_str;
pub mod helpers;
pub mod tree;

use std::collections::HashMap;

pub type Partition = HashMap<tree::TreeId, tree::Tree>;
