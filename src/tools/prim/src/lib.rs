use std::borrow::Borrow;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};

#[derive(Clone, Debug, Default, Eq)]
pub struct Param<T> {
    pub name: String,
    pub width: Option<u32>,
    pub value: T,
}

#[derive(Clone, Debug, Default, Eq)]
pub struct Port {
    pub name: String,
    pub width: u32,
}

pub type ParamSet<T> = HashSet<Param<T>>;
pub type PortSet = HashSet<Port>;

// T ~> Param value type
#[derive(Clone, Debug, Default)]
pub struct Prim<T> {
    pub name: String,
    pub param: ParamSet<T>,
    pub input: PortSet,
    pub output: PortSet,
}

impl<T> PartialEq for Param<T> {
    fn eq(&self, other: &Param<T>) -> bool {
        self.name == other.name
    }
}

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

impl<T: Eq + Default> Prim<T> {
    pub fn new() -> Self {
        Prim::default()
    }
    pub fn name(&self) -> String {
        self.name.to_string()
    }
    pub fn param_set(&self) -> &ParamSet<T> {
        &self.param
    }
    pub fn input_set(&self) -> &PortSet {
        &self.input
    }
    pub fn output_set(&self) -> &PortSet {
        &self.output
    }
    pub fn set_name<S>(&mut self, name: S)
    where
        S: AsRef<str>,
    {
        self.name = name.as_ref().to_string();
    }
    pub fn set_param(&mut self, param: Param<T>) -> bool {
        self.param.insert(param)
    }
    pub fn set_input<S>(&mut self, input: Port) -> bool {
        self.input.insert(input)
    }
    pub fn set_output<S>(&mut self, output: Port) -> bool {
        self.output.insert(output)
    }
}
