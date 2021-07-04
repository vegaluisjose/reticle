pub mod borrow;
pub mod eq;
pub mod from;
pub mod hash;
pub mod helpers;
pub mod partial_eq;
mod to_prim;

pub use crate::prim::to_prim::ToPrim;

use derive_more::{Deref, DerefMut};
use std::collections::HashSet;

#[derive(Clone, Debug, Default, Eq)]
pub struct Port {
    pub name: String,
    pub width: u32,
}

#[derive(Clone, Debug, Default, Eq)]
pub struct Param<T> {
    pub name: String,
    pub value: T,
    pub width: Option<u32>,
}

#[derive(Clone, Debug, Default, Deref, DerefMut)]
pub struct ParamSet<T>(HashSet<Param<T>>);

#[derive(Clone, Debug, Default, PartialEq, Deref, DerefMut)]
pub struct PortSet(HashSet<Port>);

// T ~> Param value type, every primitive is free to define T
#[derive(Clone, Debug)]
pub struct Prim<T> {
    pub name: String,
    pub param: ParamSet<T>,
    pub input: PortSet,
    pub output: PortSet,
}

#[derive(thiserror::Error, Debug)]
pub enum PrimError {
    #[error("Invalid parameter value: {0}")]
    InvalidParamValue(String),
    #[error("Missing parameter: {0}")]
    MissingParam(String),
}
