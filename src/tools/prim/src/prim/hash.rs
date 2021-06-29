use crate::prim::{Param, Port};
use std::hash::{Hash, Hasher};

impl<T> Hash for Param<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl Hash for Port {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}
