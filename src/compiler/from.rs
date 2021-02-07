use crate::compiler::tree::*;
use crate::ir::ast::*;

impl From<OpWire> for NodeOp {
    fn from(input: OpWire) -> Self {
        NodeOp::Wire(input)
    }
}

impl From<OpComp> for NodeOp {
    fn from(input: OpComp) -> Self {
        NodeOp::Comp(input)
    }
}