use crate::passes::map::tree::{Tree, TreeId};
use std::collections::HashMap;

pub type Partition = HashMap<TreeId, Tree>;
