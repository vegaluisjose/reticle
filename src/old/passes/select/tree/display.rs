use crate::passes::select::tree::*;
use petgraph::dot::{Config, Dot};
use std::fmt;

impl fmt::Display for TreeOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let op = match self {
            TreeOp::Input => "in".to_string(),
            TreeOp::Prim(op) => op.to_string(),
        };
        write!(f, "{}", op)
    }
}

impl fmt::Display for TreeNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(tile) = self.tile() {
            write!(
                f,
                "{} - {} - {} - {}",
                self.id(),
                self.ty(),
                self.op(),
                tile.loc()
            )
        } else {
            write!(f, "{} - {} - {}", self.id(), self.ty(), self.op())
        }
    }
}

impl fmt::Display for TreeEdge {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "")
    }
}

impl fmt::Display for Tree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            Dot::with_config(&self.graph, &[Config::EdgeNoLabel])
        )
    }
}