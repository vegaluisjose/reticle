use std::collections::HashMap;

// use 64-bit values for now, probably this
// should change if we would like to support
// bigger int types
pub type Value = i64;
pub type Map = HashMap<String, Value>;

#[derive(Clone, Debug)]
pub struct State {
    inputs: Map,
}

impl Default for State {
    fn default() -> State {
        State { inputs: Map::new() }
    }
}

impl State {
    pub fn add_input(&mut self, id: &str, value: Value) {
        self.inputs.insert(id.to_string(), value);
    }

    pub fn get_value(&self, id: &str) -> Value {
        if let Some(input) = self.inputs.get(id) {
            *input
        } else {
            panic!("id:{} not found", id);
        }
    }
}
