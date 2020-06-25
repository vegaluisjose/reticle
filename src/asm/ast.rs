use crate::util::pretty::PrettyPrinter;

pub type Id = String;

#[derive(Clone, Debug)]
pub enum DataType {
    UInt(u64),
}

#[derive(Clone, Debug)]
pub enum Expr {
    ULit(u64),
    SLit(i64),
    Str(Id),
    Attr(Rc<Expr>, Rc<Expr>),
}

#[derive(Clone, Debug)]
pub enum Port {
    Input {
        id: Id,
        datatype: DataType,
    },
    Output {
        id: Id,
        datatype: DataType,
    },
}

#[derive(Clone, Debug)]
pub struct Prim {
    name: Id,
    outputs: Vec<Port>,
}

#[derive(Clone, Debug)]
pub struct Asm {
    name: Id,
    attrs: Vec<Expr>
    inputs: Vec<Port>,
    outputs: Vec<Port>,
    body: Vec<Prim>,
}
