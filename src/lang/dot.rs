use crate::util::pretty_print::{add_newline, block_with_braces, PrettyPrint};
use pretty::RcDoc;

#[derive(Clone, Debug)]
pub enum Shape {
    Octagon,
    Record,
}

#[derive(Clone, Debug)]
pub enum Dir {
    Input,
    Output,
}

#[derive(Clone, Debug)]
pub enum Port {
    Input { name: String, shape: Shape },
    Output { name: String, shape: Shape },
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
    pub ports: Vec<Port>,
    pub blocks: Vec<Block>,
}

impl Port {
    pub fn new_input(name: &str) -> Port {
        Port::Input {
            name: name.to_string(),
            shape: Shape::Octagon,
        }
    }

    pub fn new_output(name: &str) -> Port {
        Port::Output {
            name: name.to_string(),
            shape: Shape::Octagon,
        }
    }

    pub fn shape(&self) -> &Shape {
        match self {
            Port::Input { name: _, shape } => shape,
            Port::Output { name: _, shape } => shape,
        }
    }

    pub fn name(&self) -> String {
        match self {
            Port::Input { name, shape: _ } => name.to_string(),
            Port::Output { name, shape: _ } => name.to_string(),
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

    pub fn add_input(&mut self, id: &str, name: &str) {
        self.inputs.push(Pin::new(id, name));
    }

    pub fn add_output(&mut self, id: &str, name: &str) {
        self.outputs.push(Pin::new(id, name));
    }
}

impl Graph {
    pub fn new(name: &str) -> Graph {
        Graph {
            name: name.to_string(),
            ports: Vec::new(),
            blocks: Vec::new(),
        }
    }

    pub fn add_input(&mut self, id: &str) {
        self.ports.push(Port::new_input(id))
    }

    pub fn add_output(&mut self, id: &str) {
        self.ports.push(Port::new_output(id))
    }

    pub fn add_block(&mut self, block: Block) {
        self.blocks.push(block);
    }

    pub fn ports(&self) -> &Vec<Port> {
        &self.ports
    }
}

impl PrettyPrint for Port {
    fn to_doc(&self) -> RcDoc<()> {
        RcDoc::as_string(&self.name())
            .append(RcDoc::space())
            .append(RcDoc::text("["))
            .append(RcDoc::text("shape"))
            .append(RcDoc::text("="))
            .append(self.shape().to_doc())
            .append(RcDoc::text(","))
            .append(RcDoc::space())
            .append(RcDoc::text("label"))
            .append(RcDoc::text("="))
            .append(RcDoc::text(r#"""#))
            .append(RcDoc::as_string(&self.name()))
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
        let ports = add_newline(
            self.ports()
                .iter()
                .map(|x| x.to_doc().append(RcDoc::text(";"))),
        );
        let name = RcDoc::text("digraph")
            .append(RcDoc::space())
            .append(RcDoc::as_string(&self.name));
        block_with_braces(name, ports)
    }
}

pub fn example() {
    let mut block = Block::new("t0", "add");
    block.add_input("lhs", "L");
    block.add_input("rhs", "R");
    block.add_output("out", "O");
    let mut graph = Graph::new("muladd");
    graph.add_input("a");
    graph.add_input("b");
    graph.add_output("y");
    graph.add_block(block);
    println!("{}", graph.to_pretty());
}
