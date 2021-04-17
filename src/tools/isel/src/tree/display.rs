use crate::tree::*;
use std::fmt;

impl fmt::Display for NodeOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NodeOp::Wire(wire) => write!(f, "{}", wire),
            NodeOp::Prim(comp) => write!(f, "{}", comp),
            NodeOp::Inp => write!(f, "inp"),
        }
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}-{}-{}-{}",
            self.ty(),
            self.op(),
            self.attr(),
            self.prim()
        )
    }
}

impl fmt::Display for Tree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut digraph = String::from("digraph {\n");
        // declare nodes
        for i in 0..self.index {
            if let Some(node) = self.node.get(&i) {
                let label = format!("{} [ label = \"{}\" ]\n", i, node);
                digraph.push_str(&label);
            }
        }
        // declare edges
        for i in 0..self.index {
            if let Some(edges) = self.edge.get(&i) {
                for e in edges {
                    let edge = format!("{} -> {} [ ]\n", i, e);
                    digraph.push_str(&edge);
                }
            }
        }
        digraph.push('}');
        write!(f, "{}", digraph)
    }
}
