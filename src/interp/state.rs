use crate::interp::ty::{Map, Value};

#[derive(Clone, Debug)]
pub struct State {
    inputs: Map,
    regs: Map,
    temps: Map,
}

impl Default for State {
    fn default() -> State {
        State {
            inputs: Map::new(),
            regs: Map::new(),
            temps: Map::new(),
        }
    }
}

impl State {
    pub fn add_input(&mut self, id: &str, value: Value) {
        self.inputs.insert(id.to_string(), value);
    }

    pub fn add_reg(&mut self, id: &str, value: Value) {
        self.regs.insert(id.to_string(), value);
    }

    pub fn add_temp(&mut self, id: &str, value: Value) {
        self.temps.insert(id.to_string(), value);
    }

    pub fn is_reg(&self, id: &str) -> bool {
        self.regs.contains_key(id)
    }

    pub fn inputs(&self) -> &Map {
        &self.inputs
    }

    pub fn regs(&self) -> &Map {
        &self.regs
    }

    pub fn temps(&self) -> &Map {
        &self.temps
    }

    pub fn get(&self, id: &str) -> Value {
        if let Some(input) = self.inputs.get(id) {
            *input
        } else if let Some(reg) = self.regs.get(id) {
            *reg
        } else if let Some(temp) = self.temps.get(id) {
            *temp
        } else {
            panic!("id:{} not found", id);
        }
    }

    pub fn get_input(&self, id: &str) -> Value {
        if let Some(input) = self.inputs.get(id) {
            *input
        } else {
            panic!("input:{} not found", id);
        }
    }

    pub fn get_reg(&self, id: &str) -> Value {
        if let Some(reg) = self.regs.get(id) {
            *reg
        } else {
            panic!("reg:{} not found", id);
        }
    }

    pub fn get_temp(&self, id: &str) -> Value {
        if let Some(temp) = self.temps.get(id) {
            *temp
        } else {
            panic!("temp:{} not found", id);
        }
    }

    pub fn update_regs_from_state(&mut self, other: &State) {
        self.regs = other.regs().clone();
    }

    pub fn contains(&self, id: &str) -> bool {
        self.inputs.get(id).is_some() || self.regs.get(id).is_some() || self.temps.get(id).is_some()
    }
}
