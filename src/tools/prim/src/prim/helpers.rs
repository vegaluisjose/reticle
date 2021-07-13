use crate::prim::{Error, Param, ParamSet, Port, PortSet, Prim};
use anyhow::Result;
use std::collections::HashSet;
use std::fmt;

impl Port {
    #[must_use]
    pub fn new(name: &str, width: u32) -> Self {
        Port {
            name: name.into(),
            width,
        }
    }
    #[must_use]
    pub fn name(&self) -> String {
        self.name.to_string()
    }
    #[must_use]
    pub fn width(&self) -> u32 {
        self.width
    }
    pub fn set_name(&mut self, name: &str) {
        self.name = name.into();
    }
    pub fn set_width(&mut self, width: u32) {
        self.width = width;
    }
}

impl<T> Param<T> {
    #[must_use]
    pub fn new(name: &str, value: T) -> Self {
        Param {
            name: name.into(),
            value,
        }
    }
    #[must_use]
    pub fn name(&self) -> String {
        self.name.to_string()
    }
    #[must_use]
    pub fn value(&self) -> &T {
        &self.value
    }
    pub fn set_name(&mut self, name: &str) {
        self.name = name.into();
    }
    pub fn set_value(&mut self, value: T) {
        self.value = value;
    }
}

impl<T> ParamSet<T> {
    #[must_use]
    pub fn new() -> Self {
        ParamSet(HashSet::new())
    }
}

impl PortSet {
    #[must_use]
    pub fn new() -> Self {
        PortSet(HashSet::new())
    }
}

impl<T: Eq + fmt::Debug + fmt::Display> Prim<T> {
    #[must_use]
    pub fn name(&self) -> String {
        self.name.to_string()
    }
    #[must_use]
    pub fn param(&self) -> &ParamSet<T> {
        &self.param
    }
    #[must_use]
    pub fn input(&self) -> &PortSet {
        &self.input
    }
    #[must_use]
    pub fn output(&self) -> &PortSet {
        &self.output
    }
    /// # Errors
    ///
    /// Will return `Err` if value U is invalid or missing
    pub fn set_param<U>(&mut self, name: &str, value: U) -> Result<()>
    where
        U: Into<T>,
    {
        if let Some(old) = self.param.get(name) {
            let value: T = value.into();
            if old.value() == &value {
                let param = Param {
                    name: old.name(),
                    value,
                };
                self.param.replace(param);
                Ok(())
            } else {
                Err(Error::InvalidParamValue(value.to_string()).into())
            }
        } else {
            Err(Error::MissingParam(name.into()).into())
        }
    }
}
