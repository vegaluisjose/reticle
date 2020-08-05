use std::collections::HashMap;

// use 64-bit values for now, probably this
// should change if we would like to support
// bigger int types
pub type Value = i64;
pub type Map = HashMap<String, Value>;
