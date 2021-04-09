use crate::tree::*;

impl From<OpWire> for NodeOp {
    fn from(input: OpWire) -> Self {
        NodeOp::Wire(input)
    }
}

impl From<OpPrim> for NodeOp {
    fn from(input: OpPrim) -> Self {
        NodeOp::Prim(input)
    }
}
