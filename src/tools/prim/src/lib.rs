pub mod ultrascale;

use anyhow::Result;
use derive_more::{Deref, DerefMut};
use std::borrow::Borrow;
use std::collections::HashSet;
use std::fmt;
use std::hash::{Hash, Hasher};
use thiserror::Error;

#[derive(Clone, Debug, Default, Eq)]
pub struct Param<T> {
    pub name: String,
    pub value: T,
    pub width: Option<u32>,
}

#[derive(Clone, Debug, Default, Eq)]
pub struct Port {
    pub name: String,
    pub width: u32,
}

#[derive(Clone, Debug, Default, Deref, DerefMut)]
pub struct ParamSet<T>(HashSet<Param<T>>);

#[derive(Clone, Debug, Default, PartialEq, Deref, DerefMut)]
pub struct PortSet(HashSet<Port>);

// T ~> Param value type
#[derive(Clone, Debug)]
pub struct Prim<T> {
    pub name: String,
    pub param: ParamSet<T>,
    pub input: PortSet,
    pub output: PortSet,
}

#[derive(Error, Debug)]
pub enum PrimError {
    #[error("Invalid parameter value: {0}")]
    InvalidParamValue(String),
    #[error("Missing parameter: {0}")]
    MissingParam(String),
}

pub trait ToPrim<T> {
    fn to_name(&self) -> String;
    fn to_param(&self) -> ParamSet<T>;
    fn to_input(&self) -> PortSet;
    fn to_output(&self) -> PortSet;
    fn to_prim(&self) -> Prim<T> {
        Prim {
            name: self.to_name(),
            param: self.to_param(),
            input: self.to_input(),
            output: self.to_output(),
        }
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

impl<T> Eq for ParamSet<T> where T: Eq + Hash {}

impl<T> Hash for Param<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
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

impl PartialEq for Port {
    fn eq(&self, other: &Port) -> bool {
        self.name == other.name
    }
}

impl Hash for Port {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

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

impl<T: Eq + Default + fmt::Debug + fmt::Display> Prim<T> {
    pub fn new() -> Self {
        Prim {
            name: String::new(),
            param: ParamSet::new(),
            input: PortSet::new(),
            output: PortSet::new(),
        }
    }
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

impl From<&(&str, u32)> for Port {
    fn from(input: &(&str, u32)) -> Self {
        Port {
            name: input.0.into(),
            width: input.1,
        }
    }
}

impl From<(&str, u32)> for Port {
    fn from(input: (&str, u32)) -> Self {
        Port {
            name: input.0.into(),
            width: input.1,
        }
    }
}

impl From<&[(&str, u32)]> for PortSet {
    fn from(input: &[(&str, u32)]) -> Self {
        let mut set = PortSet::new();
        for t in input {
            set.insert(t.into());
        }
        set
    }
}

impl<T: Clone> From<&(&str, T)> for Param<T> {
    fn from(input: &(&str, T)) -> Self {
        Param {
            name: input.0.into(),
            value: input.1.clone(),
            width: None,
        }
    }
}

impl<T> From<(&str, T)> for Param<T> {
    fn from(input: (&str, T)) -> Self {
        Param {
            name: input.0.into(),
            value: input.1,
            width: None,
        }
    }
}
