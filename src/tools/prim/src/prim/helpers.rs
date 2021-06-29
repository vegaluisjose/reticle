use crate::prim::{Param, ParamSet, Port, PortSet, Prim, PrimError};
use anyhow::Result;
use std::collections::HashSet;
use std::fmt;

impl Port {
    pub fn new(name: &str, width: u32) -> Self {
        Port {
            name: name.into(),
            width,
        }
    }
    pub fn name(&self) -> String {
        self.name.to_string()
    }
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
    pub fn new(name: &str, value: T) -> Self {
        Param {
            name: name.into(),
            width: None,
            value,
        }
    }
    pub fn name(&self) -> String {
        self.name.to_string()
    }
    pub fn width(&self) -> Option<u32> {
        self.width
    }
    pub fn value(&self) -> &T {
        &self.value
    }
    pub fn set_name(&mut self, name: &str) {
        self.name = name.into();
    }
    pub fn set_width(&mut self, width: u32) {
        self.width = Some(width);
    }
    pub fn set_value(&mut self, value: T) {
        self.value = value;
    }
}

impl<T> ParamSet<T> {
    pub fn new() -> Self {
        ParamSet(HashSet::new())
    }
}

impl PortSet {
    pub fn new() -> Self {
        PortSet(HashSet::new())
    }
}

impl<T: Eq + Default + fmt::Debug + fmt::Display> Prim<T> {
    pub fn name(&self) -> String {
        self.name.to_string()
    }
    pub fn param(&self) -> &ParamSet<T> {
        &self.param
    }
    pub fn input(&self) -> &PortSet {
        &self.input
    }
    pub fn output(&self) -> &PortSet {
        &self.output
    }
    pub fn set_param<U>(&mut self, name: &str, value: U) -> Result<()>
    where
        U: Into<T>,
    {
        if let Some(old) = self.param.get(name) {
            let value: T = value.into();
            if old.value() == &value {
                let param = Param {
                    name: old.name(),
                    width: old.width(),
                    value,
                };
                self.param.replace(param);
                Ok(())
            } else {
                Err(PrimError::InvalidParamValue(value.to_string()).into())
            }
        } else {
            Err(PrimError::MissingParam(name.into()).into())
        }
    }
}
