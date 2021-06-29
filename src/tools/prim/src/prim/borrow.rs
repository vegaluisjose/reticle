use crate::prim::{Param, Port};
use std::borrow::Borrow;

impl Borrow<String> for Port {
    fn borrow(&self) -> &String {
        &self.name
    }
}

impl Borrow<str> for Port {
    fn borrow(&self) -> &str {
        &self.name.as_str()
    }
}

impl<T> Borrow<String> for Param<T> {
    fn borrow(&self) -> &String {
        &self.name
    }
}

impl<T> Borrow<str> for Param<T> {
    fn borrow(&self) -> &str {
        &self.name.as_str()
    }
}
