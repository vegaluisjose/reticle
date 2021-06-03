use crate::errors::Error;
use std::collections::HashMap;

pub type ParamMap<T> = HashMap<String, T>;

#[derive(Clone, Debug)]
pub struct Param<T> {
    pub map: ParamMap<T>,
}

impl<T: std::cmp::PartialEq> Param<T> {
    pub fn param(&self) -> &ParamMap<T> {
        &self.map
    }
    pub fn set_param<U: Clone>(&mut self, name: &str, value: U) -> Result<(), Error>
    where
        U: Into<T>,
    {
        if let Some(cur) = self.map.get_mut(name) {
            let val: T = value.into();
            if *cur != val {
                Err(Error::new_xpand_error("invalid param type"))
            } else {
                *cur = val;
                Ok(())
            }
        } else {
            Err(Error::new_xpand_error("param does not exists"))
        }
    }
}
