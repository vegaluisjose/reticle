use crate::util::pretty_print::{
    add_newline, add_space, block_with_braces, PrettyHelper, PrettyPrint,
};
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
pub enum RDir {
    TopBottom,
    LeftRight,
}

#[derive(Clone, Debug)]
pub enum Opt {
    Remincross(bool),
    RankDir(RDir),
    Label(String),
}

#[derive(Clone, Debug)]
pub struct Dot {
    pub name: String,
    pub ports: Vec<Port>,
    pub blocks: Vec<Block>,
    pub opts: Vec<Opt>,
    pub has_input: bool,
    pub has_output: bool,
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

    pub fn is_input(&self) -> bool {
        match self {
            Port::Input { name: _, shape: _ } => true,
            Port::Output { name: _, shape: _ } => false,
        }
    }

    pub fn is_output(&self) -> bool {
        match self {
            Port::Input { name: _, shape: _ } => false,
            Port::Output { name: _, shape: _ } => true,
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

impl Opt {
    pub fn new_label(name: &str) -> Opt {
        Opt::Label(name.to_string())
    }

    pub fn new_rankdir_tb() -> Opt {
        Opt::RankDir(RDir::TopBottom)
    }

    pub fn new_rankdir_lr() -> Opt {
        Opt::RankDir(RDir::LeftRight)
    }
}

impl Dot {
    pub fn new(name: &str) -> Dot {
        let mut default_opt: Vec<Opt> = Vec::new();
        default_opt.push(Opt::new_label(name));
        default_opt.push(Opt::new_rankdir_tb());
        Dot {
            name: name.to_string(),
            ports: Vec::new(),
            blocks: Vec::new(),
            opts: default_opt.to_vec(),
            has_input: false,
            has_output: false,
        }
    }

    pub fn add_input(&mut self, id: &str) {
        self.ports.push(Port::new_input(id));
        self.has_input = true;
    }

    pub fn add_output(&mut self, id: &str) {
        self.ports.push(Port::new_output(id));
        self.has_output = true;
    }

    pub fn add_block(&mut self, block: Block) {
        self.blocks.push(block);
    }

    pub fn add_opt(&mut self, opt: Opt) {
        self.opts.push(opt);
    }

    pub fn ports(&self) -> &Vec<Port> {
        &self.ports
    }

    pub fn opts(&self) -> &Vec<Opt> {
        &self.opts
    }

    pub fn blocks(&self) -> &Vec<Block> {
        &self.blocks
    }

    pub fn has_input(&self) -> bool {
        self.has_input
    }

    pub fn has_output(&self) -> bool {
        self.has_output
    }

    pub fn has_opt(&self) -> bool {
        !self.opts.is_empty()
    }

    pub fn has_port(&self) -> bool {
        !self.ports.is_empty()
    }

    pub fn has_block(&self) -> bool {
        !self.blocks.is_empty()
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

impl PrettyPrint for RDir {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            RDir::TopBottom => RcDoc::text("TB"),
            RDir::LeftRight => RcDoc::text("LR"),
        }
    }
}

impl PrettyPrint for Opt {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            Opt::Remincross(flag) => RcDoc::text("remincross")
                .append(RcDoc::text("="))
                .append(RcDoc::as_string(flag)),
            Opt::RankDir(dir) => RcDoc::text("rankdir")
                .append(RcDoc::text("="))
                .append(dir.to_doc()),
            Opt::Label(name) => RcDoc::text("label")
                .append(RcDoc::text("="))
                .append(RcDoc::as_string(name)),
        }
    }
}

pub fn rank_source_from_ports(ports: &[Port]) -> RcDoc<()> {
    RcDoc::text("rank")
        .append(RcDoc::text("="))
        .append(RcDoc::text("source"))
        .append(RcDoc::text(";"))
        .append(RcDoc::space())
        .append(add_space(
            ports
                .iter()
                .filter(|x| x.is_input())
                .map(|x| RcDoc::as_string(&x.name()).append(RcDoc::text(";"))),
        ))
        .braces()
}

pub fn rank_sink_from_ports(ports: &[Port]) -> RcDoc<()> {
    RcDoc::text("rank")
        .append(RcDoc::text("="))
        .append(RcDoc::text("sink"))
        .append(RcDoc::text(";"))
        .append(RcDoc::space())
        .append(add_space(
            ports
                .iter()
                .filter(|x| x.is_output())
                .map(|x| RcDoc::as_string(&x.name()).append(RcDoc::text(";"))),
        ))
        .braces()
}

impl PrettyPrint for Dot {
    fn to_doc(&self) -> RcDoc<()> {
        let opts = if self.has_opt() {
            add_newline(
                self.opts()
                    .iter()
                    .map(|x| x.to_doc().append(RcDoc::text(";"))),
            )
            .append(RcDoc::hardline())
        } else {
            RcDoc::nil()
        };
        let ports = if self.has_port() {
            add_newline(
                self.ports()
                    .iter()
                    .map(|x| x.to_doc().append(RcDoc::text(";"))),
            )
            .append(RcDoc::hardline())
        } else {
            RcDoc::nil()
        };
        let rank_source = if self.has_input() {
            rank_source_from_ports(&self.ports).append(RcDoc::hardline())
        } else {
            RcDoc::nil()
        };
        let rank_sink = if self.has_output() {
            rank_sink_from_ports(&self.ports).append(RcDoc::hardline())
        } else {
            RcDoc::nil()
        };
        let blocks = if self.has_block() {
            add_newline(
                self.blocks()
                    .iter()
                    .map(|x| x.to_doc().append(RcDoc::text(";"))),
            )
        } else {
            RcDoc::nil()
        };
        let name = RcDoc::text("digraph")
            .append(RcDoc::space())
            .append(RcDoc::as_string(&self.name));
        let body = opts
            .append(ports)
            .append(rank_source)
            .append(rank_sink)
            .append(blocks);
        block_with_braces(name, body)
    }
}

pub fn example() {
    let mut block = Block::new("t0", "add");
    block.add_input("lhs", "L");
    block.add_input("rhs", "R");
    block.add_output("out", "O");
    let mut graph = Dot::new("muladd");
    graph.add_input("a");
    graph.add_input("b");
    graph.add_output("y");
    graph.add_block(block);
    graph.add_opt(Opt::Remincross(true));
    println!("{}", graph.to_pretty());
}
