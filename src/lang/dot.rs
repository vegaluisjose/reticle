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
pub enum IO {
    Input { name: String, shape: Shape },
    Output { name: String, shape: Shape },
}

#[derive(Clone, Debug)]
pub struct Port {
    pub id: String,
    pub name: String,
}

#[derive(Clone, Debug)]
pub struct Block {
    pub name: String,
    pub op: String,
    pub inputs: Vec<Port>,
    pub outputs: Vec<Port>,
    pub shape: Shape,
}

#[derive(Clone, Debug)]
pub enum Cardinal {
    North,
    South,
    East,
    West,
}

#[derive(Clone, Debug)]
pub enum Connection {
    PortToIO {
        block_id: String,
        port_id: String,
        port_dir: Cardinal,
        io_id: String,
        io_dir: Cardinal,
    },
    IOToPort {
        io_id: String,
        io_dir: Cardinal,
        block_id: String,
        port_id: String,
        port_dir: Cardinal,
    },
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
    pub io: Vec<IO>,
    pub blocks: Vec<Block>,
    pub opts: Vec<Opt>,
    pub connections: Vec<Connection>,
    pub has_input: bool,
    pub has_output: bool,
}

impl IO {
    pub fn new_input(name: &str) -> IO {
        IO::Input {
            name: name.to_string(),
            shape: Shape::Octagon,
        }
    }

    pub fn new_output(name: &str) -> IO {
        IO::Output {
            name: name.to_string(),
            shape: Shape::Octagon,
        }
    }

    pub fn shape(&self) -> &Shape {
        match self {
            IO::Input { name: _, shape } => shape,
            IO::Output { name: _, shape } => shape,
        }
    }

    pub fn name(&self) -> String {
        match self {
            IO::Input { name, shape: _ } => name.to_string(),
            IO::Output { name, shape: _ } => name.to_string(),
        }
    }

    pub fn is_input(&self) -> bool {
        match self {
            IO::Input { name: _, shape: _ } => true,
            IO::Output { name: _, shape: _ } => false,
        }
    }

    pub fn is_output(&self) -> bool {
        match self {
            IO::Input { name: _, shape: _ } => false,
            IO::Output { name: _, shape: _ } => true,
        }
    }
}

impl Port {
    pub fn new(id: &str, name: &str) -> Port {
        Port {
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
        self.inputs.push(Port::new(id, name));
    }

    pub fn add_output(&mut self, id: &str, name: &str) {
        self.outputs.push(Port::new(id, name));
    }
}

impl Connection {
    pub fn new_port_to_io_tb(block: &str, port: &str, io: &str) -> Connection {
        Connection::PortToIO {
            block_id: block.to_string(),
            port_id: port.to_string(),
            port_dir: Cardinal::South,
            io_id: io.to_string(),
            io_dir: Cardinal::North,
        }
    }

    pub fn new_io_to_port_tb(io: &str, block: &str, port: &str) -> Connection {
        Connection::IOToPort {
            io_id: io.to_string(),
            io_dir: Cardinal::South,
            block_id: block.to_string(),
            port_id: port.to_string(),
            port_dir: Cardinal::North,
        }
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
            io: Vec::new(),
            blocks: Vec::new(),
            opts: default_opt.to_vec(),
            connections: Vec::new(),
            has_input: false,
            has_output: false,
        }
    }

    pub fn add_input(&mut self, id: &str) {
        self.io.push(IO::new_input(id));
        self.has_input = true;
    }

    pub fn add_output(&mut self, id: &str) {
        self.io.push(IO::new_output(id));
        self.has_output = true;
    }

    pub fn add_block(&mut self, block: Block) {
        self.blocks.push(block);
    }

    pub fn add_opt(&mut self, opt: Opt) {
        self.opts.push(opt);
    }

    pub fn connect_port_to_io(&mut self, block: &str, port: &str, io: &str) {
        self.connections
            .push(Connection::new_port_to_io_tb(block, port, io));
    }

    pub fn connect_io_to_port(&mut self, io: &str, block: &str, port: &str) {
        self.connections
            .push(Connection::new_io_to_port_tb(io, block, port));
    }

    pub fn io(&self) -> &Vec<IO> {
        &self.io
    }

    pub fn opts(&self) -> &Vec<Opt> {
        &self.opts
    }

    pub fn blocks(&self) -> &Vec<Block> {
        &self.blocks
    }

    pub fn connections(&self) -> &Vec<Connection> {
        &self.connections
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
        !self.io.is_empty()
    }

    pub fn has_block(&self) -> bool {
        !self.blocks.is_empty()
    }

    pub fn has_connections(&self) -> bool {
        !self.connections.is_empty()
    }
}

impl PrettyPrint for IO {
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

impl PrettyPrint for Port {
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

impl PrettyPrint for Cardinal {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            Cardinal::North => RcDoc::text("n"),
            Cardinal::South => RcDoc::text("s"),
            Cardinal::East => RcDoc::text("e"),
            Cardinal::West => RcDoc::text("w"),
        }
    }
}

impl PrettyPrint for Connection {
    fn to_doc(&self) -> RcDoc<()> {
        let opt = RcDoc::text("label")
            .append(RcDoc::text("="))
            .append(RcDoc::text(r#""""#))
            .brackets();
        match self {
            Connection::PortToIO {
                block_id,
                port_id,
                port_dir,
                io_id,
                io_dir,
            } => RcDoc::as_string(block_id)
                .append(RcDoc::text(":"))
                .append(RcDoc::as_string(port_id))
                .append(RcDoc::text(":"))
                .append(port_dir.to_doc())
                .append(RcDoc::text("->"))
                .append(RcDoc::as_string(io_id))
                .append(RcDoc::text(":"))
                .append(io_dir.to_doc())
                .append(RcDoc::space())
                .append(opt),
            Connection::IOToPort {
                io_id,
                io_dir,
                block_id,
                port_id,
                port_dir,
            } => RcDoc::as_string(io_id)
                .append(RcDoc::text(":"))
                .append(io_dir.to_doc())
                .append(RcDoc::text("->"))
                .append(RcDoc::as_string(block_id))
                .append(RcDoc::text(":"))
                .append(RcDoc::as_string(port_id))
                .append(RcDoc::text(":"))
                .append(port_dir.to_doc())
                .append(RcDoc::space())
                .append(opt),
        }
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

pub fn rank_source_from_ports(io: &[IO]) -> RcDoc<()> {
    RcDoc::text("rank")
        .append(RcDoc::text("="))
        .append(RcDoc::text("source"))
        .append(RcDoc::text(";"))
        .append(RcDoc::space())
        .append(add_space(
            io.iter()
                .filter(|x| x.is_input())
                .map(|x| RcDoc::as_string(&x.name()).append(RcDoc::text(";"))),
        ))
        .braces()
}

pub fn rank_sink_from_ports(io: &[IO]) -> RcDoc<()> {
    RcDoc::text("rank")
        .append(RcDoc::text("="))
        .append(RcDoc::text("sink"))
        .append(RcDoc::text(";"))
        .append(RcDoc::space())
        .append(add_space(
            io.iter()
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
        let io = if self.has_port() {
            add_newline(
                self.io()
                    .iter()
                    .map(|x| x.to_doc().append(RcDoc::text(";"))),
            )
            .append(RcDoc::hardline())
        } else {
            RcDoc::nil()
        };
        let rank_source = if self.has_input() && self.has_connections() {
            rank_source_from_ports(&self.io).append(RcDoc::hardline())
        } else {
            RcDoc::nil()
        };
        let rank_sink = if self.has_output() && self.has_connections() {
            rank_sink_from_ports(&self.io).append(RcDoc::hardline())
        } else {
            RcDoc::nil()
        };
        let blocks = if self.has_block() {
            add_newline(
                self.blocks()
                    .iter()
                    .map(|x| x.to_doc().append(RcDoc::text(";"))),
            )
            .append(RcDoc::hardline())
        } else {
            RcDoc::nil()
        };
        let connections = if self.has_connections() {
            add_newline(
                self.connections()
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
            .append(io)
            .append(rank_source)
            .append(rank_sink)
            .append(blocks)
            .append(connections);
        block_with_braces(name, body)
    }
}

pub fn example() {
    let mut block = Block::new("t0", "add");
    block.add_input("lhs", "L");
    block.add_input("rhs", "R");
    block.add_output("out", "O");
    let mut dot = Dot::new("muladd");
    dot.add_input("a");
    dot.add_input("b");
    dot.add_output("y");
    dot.add_block(block);
    dot.add_opt(Opt::Remincross(true));
    dot.connect_io_to_port("a", "t0", "lhs");
    dot.connect_io_to_port("b", "t0", "rhs");
    dot.connect_port_to_io("t0", "out", "y");
    println!("{}", dot.to_pretty());
}
