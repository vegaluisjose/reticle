use crate::errors::Error;
use derive_more::{Deref, DerefMut, From};
use std::collections::HashMap;

pub type ParamMap<T> = HashMap<String, T>;

#[derive(Clone, Debug, Default, From, Deref, DerefMut)]
pub struct Param<T>(ParamMap<T>);

impl<T: std::cmp::PartialEq> Param<T> {
    pub fn new() -> Param<T> {
        Param::from(ParamMap::<T>::new())
    }
    pub fn param(&self) -> &ParamMap<T> {
        &self
    }
    pub fn set_param<U: Clone>(&mut self, name: &str, value: U) -> Result<(), Error>
    where
        U: Into<T>,
    {
        if let Some(cur) = self.get_mut(name) {
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
