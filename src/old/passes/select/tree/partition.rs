use crate::passes::select::tree::{Tree, TreeId};
use std::collections::HashMap;

pub type Partition = HashMap<TreeId, Tree>;
