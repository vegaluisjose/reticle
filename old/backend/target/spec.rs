use serde::{Deserialize, Serialize};
use std::rc::Rc;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum SpecExpr {
    Input(String),
    UnOp(String, String, Rc<SpecExpr>),
    BinOp(String, String, Rc<SpecExpr>, Rc<SpecExpr>),
    TerOp(String, String, Rc<SpecExpr>, Rc<SpecExpr>, Rc<SpecExpr>),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SpecCost {
    pub delay: u32,
    pub area: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SpecInstr {
    pub name: String,
    pub cost: SpecCost,
    pub loc: String,
    pub expr: SpecExpr,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Spec {
    pub isa: Vec<SpecInstr>,
}
