pub mod display;
pub mod from;
pub mod helpers;
pub mod tree;

use std::collections::HashMap;

pub type Partition = HashMap<tree::TreeId, tree::Tree>;
