// use crate::loc::attr_from_loc;
// use crate::loc::{Bel, BelLut, ExprCoord, Loc};
// use std::collections::HashMap;
use crate::loc::Loc;
use crate::port::{Input, Output};
// use verilog::ast as vl;

#[derive(Clone, Debug)]
pub struct Attr {
    pub init: u64,
}

#[derive(Clone, Debug)]
pub struct Lut2 {
    pub name: String,
    pub prim: String,
    pub loc: Loc,
    pub attr: Attr,
    pub input: Input,
    pub output: Output,
}

impl Default for Attr {
    fn default() -> Self {
        Attr { init: 0 }
    }
}
