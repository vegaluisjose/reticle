use crate::util::pretty_print::PrettyPrint;
use pretty::RcDoc;

#[derive(Clone, Debug)]
pub enum Shape {
    Octagon,
    Record,
}

#[derive(Clone, Debug)]
pub struct Port {
    pub name: String,
    pub shape: Shape,
}

#[derive(Clone, Debug)]
pub struct Pin {
    pub id: String,
    pub name: String,
}

#[derive(Clone, Debug)]
pub struct Block {
    pub name: String,
    pub op: String,
    pub inputs: Vec<Pin>,
    pub outputs: Vec<Pin>,
    pub shape: Shape,
}

#[derive(Clone, Debug)]
pub struct Graph {
    pub name: String,
    pub inputs: Vec<Port>,
    pub outputs: Vec<Port>,
    pub blocks: Vec<Block>,
}

impl Port {
    pub fn new(name: &str) -> Port {
        Port {
            name: name.to_string(),
            shape: Shape::Octagon,
        }
    }
}

impl Pin {
    pub fn new(id: &str, name: &str) -> Pin {
        Pin {
            id: id.to_string(),
            name: name.to_string(),
        }
    }
}

impl Block {
    pub fn new(name: &str, op: &str) -> Block {
        Block {
            name: name.to_string(),
            op: op.to_string(),
            inputs: Vec::new(),
            outputs: Vec::new(),
            shape: Shape::Record,
        }
    }

    pub fn add_input_pin(&mut self, id: &str, name: &str) {
        self.inputs.push(Pin::new(id, name));
    }

    pub fn add_output_pin(&mut self, id: &str, name: &str) {
        self.outputs.push(Pin::new(id, name));
    }
}

impl Graph {
    pub fn new(name: &str) -> Graph {
        Graph {
            name: name.to_string(),
            inputs: Vec::new(),
            outputs: Vec::new(),
            blocks: Vec::new(),
        }
    }

    pub fn add_input_port(&mut self, id: &str) {
        self.inputs.push(Port::new(id))
    }

    pub fn add_output_port(&mut self, id: &str) {
        self.outputs.push(Port::new(id))
    }

    pub fn add_block(&mut self, block: Block) {
        self.blocks.push(block);
    }
}

impl PrettyPrint for Port {
    fn to_doc(&self) -> RcDoc<()> {
        RcDoc::as_string(&self.name)
            .append(RcDoc::space())
            .append(RcDoc::text("["))
            .append(RcDoc::text("shape"))
            .append(RcDoc::text("="))
            .append(self.shape.to_doc())
            .append(RcDoc::text(","))
            .append(RcDoc::space())
            .append(RcDoc::text("label"))
            .append(RcDoc::text("="))
            .append(RcDoc::text(r#"""#))
            .append(RcDoc::as_string(&self.name))
            .append(RcDoc::text(r#"""#))
            .append(RcDoc::text("]"))
    }
}

impl PrettyPrint for Shape {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            Shape::Octagon => RcDoc::text("octagon"),
            Shape::Record => RcDoc::text("record"),
        }
    }
}

impl PrettyPrint for Pin {
    fn to_doc(&self) -> RcDoc<()> {
        RcDoc::text("<")
            .append(RcDoc::as_string(&self.id))
            .append(RcDoc::text(">"))
            .append(RcDoc::space())
            .append(RcDoc::as_string(&self.name))
    }
}

impl PrettyPrint for Block {
    fn to_doc(&self) -> RcDoc<()> {
        let input_doc =
            RcDoc::intersperse(self.inputs.iter().map(|i| i.to_doc()), RcDoc::text("|"));
        let output_doc =
            RcDoc::intersperse(self.outputs.iter().map(|o| o.to_doc()), RcDoc::text("|"));
        let label_doc = match (self.inputs.is_empty(), self.outputs.is_empty()) {
            (true, true) => RcDoc::as_string(&self.op),
            (true, _) => RcDoc::text("{")
                .append(RcDoc::as_string(&self.op))
                .append(RcDoc::text("|"))
                .append(RcDoc::text("{"))
                .append(output_doc)
                .append(RcDoc::text("}"))
                .append(RcDoc::text("}")),
            (_, true) => RcDoc::text("{")
                .append(RcDoc::text("{"))
                .append(input_doc)
                .append(RcDoc::text("}"))
                .append(RcDoc::text("|"))
                .append(RcDoc::as_string(&self.op))
                .append(RcDoc::text("}")),
            (_, _) => RcDoc::text("{")
                .append(RcDoc::text("{"))
                .append(input_doc)
                .append(RcDoc::text("}"))
                .append(RcDoc::text("|"))
                .append(RcDoc::as_string(&self.op))
                .append(RcDoc::text("|"))
                .append(RcDoc::text("{"))
                .append(output_doc)
                .append(RcDoc::text("}"))
                .append(RcDoc::text("}")),
        };
        RcDoc::as_string(&self.name)
            .append(RcDoc::space())
            .append(RcDoc::text("["))
            .append(RcDoc::text("shape"))
            .append(RcDoc::text("="))
            .append(self.shape.to_doc())
            .append(RcDoc::text(","))
            .append(RcDoc::space())
            .append(RcDoc::text("label"))
            .append(RcDoc::text("="))
            .append(RcDoc::text(r#"""#))
            .append(label_doc)
            .append(RcDoc::text(r#"""#))
            .append(RcDoc::text("]"))
    }
}

impl PrettyPrint for Graph {
    fn to_doc(&self) -> RcDoc<()> {
        RcDoc::text("digraph")
            .append(RcDoc::space())
            .append(RcDoc::as_string(&self.name))
            .append(RcDoc::space())
            .append(RcDoc::text("{"))
            .append(RcDoc::text("}"))
    }
}

pub fn example() {
    let graph = Graph::new("muladd");
    let port = Port::new("a");
    let mut block = Block::new("t0", "add");
    block.add_input_pin("lhs", "L");
    block.add_input_pin("rhs", "R");
    block.add_output_pin("out", "O");
    println!("{}", graph.to_pretty());
    println!("{}", port.to_pretty());
    println!("{}", block.to_pretty());
}
