use crate::lang::interp::ty::Value;
use std::collections::HashMap;
use std::slice::Iter;

type TraceValue = Vec<Value>;
type Map = HashMap<String, TraceValue>;

#[derive(Clone, Debug)]
pub struct Trace {
    map: Map,
}

impl Default for Trace {
    fn default() -> Trace {
        Trace { map: Map::new() }
    }
}

impl Trace {
    pub fn poke(&mut self, id: &str, value: Value) {
        if let Some(data) = self.map.get_mut(id) {
            data.push(value);
        } else {
            let mut data = Vec::new();
            data.push(value);
            self.map.insert(id.to_string(), data);
        }
    }

    pub fn is_valid(&self) -> bool {
        let mut iter = self.map.values();
        if let Some(data) = iter.next() {
            let mut current = data.len();
            while let Some(next) = iter.next() {
                if next.len() != current {
                    break;
                } else {
                    current = next.len();
                }
            }
        }
        iter.next().is_none() && !self.map.is_empty()
    }

    pub fn trace_from_id(&self, id: &str) -> &TraceValue {
        if let Some(trace) = self.map.get(id) {
            trace
        } else {
            panic!("Error: {} not found", id);
        }
    }

    pub fn iter_from_id(&self, id: &str) -> Iter<Value> {
        if let Some(trace) = self.map.get(id) {
            trace.iter()
        } else {
            panic!("Error: {} not found", id);
        }
    }

    pub fn len(&self) -> u32 {
        assert!(self.is_valid(), "Error: malformed trace");
        let mut iter = self.map.values();
        if let Some(data) = iter.next() {
            data.len() as u32
        } else {
            panic!("Error: empty queue");
        }
    }

    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }
}