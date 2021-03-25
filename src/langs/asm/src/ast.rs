use ir::ast as ir;
//use serde::{Deserialize, Serialize};
//use std::rc::Rc;

pub type Id = ir::Id;

//pub type Ty = ir::Ty;
//pub type Prim = ir::Prim;
//pub type ExprTerm = ir::ExprTerm;
//pub type ExprTup = ir::ExprTup;
//pub type Expr = ir::Expr;
//pub type OpWire = ir::OpWire;
//pub type InstrWire = ir::InstrWire;
//pub type Sig = ir::Sig;
//
//#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, Eq, Hash)]
//pub enum OpCoord {
//    Add,
//}
//
//#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, Eq, Hash)]
//pub enum ExprCoord {
//    Any,
//    Var(Id),
//    Val(u64),
//    Bin(OpCoord, Rc<ExprCoord>, Rc<ExprCoord>),
//}
//
//#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, Eq, Hash)]
//pub struct Loc {
//    pub prim: Prim,
//    pub x: ExprCoord,
//    pub y: ExprCoord,
//}
//
//#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, Eq, Hash)]
//pub enum OpAsm {
//    Op(Id),
//}
//
//#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, Eq, Hash)]
//pub struct InstrAsm {
//    pub op: OpAsm,
//    pub dst: Expr,
//    pub arg: Expr,
//    pub loc: Loc,
//}
//
//#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, Eq, Hash)]
//pub enum Instr {
//    Wire(InstrWire),
//    Asm(InstrAsm),
//}
//
//#[derive(Serialize, Deserialize, PartialEq, Default, Clone, Debug, Eq, Hash)]
//pub struct Prog {
//    pub sig: Sig,
//    pub body: Vec<Instr>,
//}
