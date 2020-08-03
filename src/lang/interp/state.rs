use crate::lang::interp::ty::{Map, Value};

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
