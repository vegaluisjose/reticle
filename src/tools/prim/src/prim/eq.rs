use crate::prim::ParamSet;
use std::hash::Hash;

impl<T> Eq for ParamSet<T> where T: Eq + Hash {}
