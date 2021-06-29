use crate::prim::{Param, ParamSet, Port};
use std::hash::Hash;

impl PartialEq for Port {
    fn eq(&self, other: &Port) -> bool {
        self.name == other.name
    }
}

impl<T> PartialEq for Param<T> {
    fn eq(&self, other: &Param<T>) -> bool {
        self.name == other.name
    }
}

impl<T> PartialEq for ParamSet<T>
where
    T: Eq + Hash,
{
    fn eq(&self, other: &ParamSet<T>) -> bool {
        self.0 == other.0
    }
}
