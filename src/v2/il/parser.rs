use crate::util::file::read_to_string;
use crate::v2::il::ast::*;
use pest_consume::{match_nodes, Error, Parser};
use std::path::Path;
use std::str::FromStr;

pub type Result<T> = std::result::Result<T, Error<Rule>>;
type Node<'i> = pest_consume::Node<'i, Rule, ()>;

const _GRAMMAR: &str = include_str!("grammar.pest");

#[derive(Parser)]
#[grammar = "v2/il/grammar.pest"]
struct ILParser;

#[pest_consume::parser]
impl ILParser {
    fn EOI(_input: Node) -> Result<()> {
        Ok(())
    }

    fn id(input: Node) -> Result<Id> {
        Ok(input.as_str().to_string())
    }

    fn ty(input: Node) -> Result<Ty> {
        Ok(Ty::from_str(input.as_str()).unwrap())
    }

    fn resource(input: Node) -> Result<Prim> {
        Ok(Prim::from_str(input.as_str()).unwrap())
    }

    fn val(input: Node) -> Result<Expr> {
        let val = input.as_str().parse::<i64>().unwrap();
        Ok(Expr::Val(val))
    }

    fn name(input: Node) -> Result<Expr> {
        Ok(match_nodes!(
            input.into_children();
            [id(id), ty(ty)] => Expr::Name(id, ty),
            [id(id)] => Expr::Name(id, Ty::Var),
        ))
    }

    fn tup_name(input: Node) -> Result<Expr> {
        Ok(match_nodes!(
            input.into_children();
            [name(names)..] => Expr::from(ExprTup{ exprs: names.collect()}),
        ))
    }

    fn tup_val(input: Node) -> Result<Expr> {
        Ok(match_nodes!(
            input.into_children();
            [val(vals)..] => Expr::from(ExprTup{ exprs: vals.collect()}),
        ))
    }

    fn io(input: Node) -> Result<Expr> {
        Ok(match_nodes!(
            input.into_children();
            [name(name)] => name,
            [tup_name(tup)] => tup,
        ))
    }

    fn instr(input: Node) -> Result<Instr> {
        let instr = input.as_str().to_string();
        Ok(match_nodes!(
            input.into_children();
            [io(dst), id(opcode), tup_val(attrs)] => {
                let wire = WireOp::from_str(&opcode);
                let comp = CompOp::from_str(&opcode);
                match (wire, comp) {
                    (Ok(op), Err(_)) => Instr::from(
                        InstrWire {
                            op,
                            dst,
                            attrs,
                            args: Expr::Tup(ExprTup::default()),
                        }
                    ),
                    (_, _) => panic!(format!("Error: ~~~{}~~~ is not valid instruction", instr))
                }
            },
            [io(dst), id(opcode), io(args)] => {
                let wire = WireOp::from_str(&opcode);
                let comp = CompOp::from_str(&opcode);
                match (wire, comp) {
                    (Ok(op), Err(_)) => Instr::from(
                        InstrWire {
                            op,
                            dst,
                            attrs: Expr::Tup(ExprTup::default()),
                            args,
                        }
                    ),
                    (Err(_), Ok(op)) => Instr::from(
                        InstrComp {
                            op,
                            dst,
                            attrs: Expr::Tup(ExprTup::default()),
                            args,
                            prim: Prim::Var,
                        }
                    ),
                    (Err(_), Err(_)) => Instr::from(
                        InstrCall {
                            op: CallOp::from_str(&opcode).unwrap(),
                            dst,
                            args,
                        }
                    ),
                    (_, _) => panic!(format!("Error: ~~~{}~~~ is not valid instruction", instr))
                }
            },
            [io(dst), id(opcode), tup_val(attrs), io(args)] => {
                let wire = WireOp::from_str(&opcode);
                let comp = CompOp::from_str(&opcode);
                match (wire, comp) {
                    (Ok(op), Err(_)) => Instr::from(
                        InstrWire {
                            op,
                            dst,
                            attrs,
                            args,
                        }
                    ),
                    (Err(_), Ok(op)) => Instr::from(
                        InstrComp {
                            op,
                            dst,
                            attrs,
                            args,
                            prim: Prim::Var,
                        }
                    ),
                    (_, _) => panic!(format!("Error: ~~~{}~~~ is not valid instruction", instr))
                }
            },
            [io(dst), id(opcode), io(args), resource(prim)] => {
                let comp = CompOp::from_str(&opcode);
                Instr::from(InstrComp {
                    op: comp.unwrap(),
                    dst,
                    attrs: Expr::Tup(ExprTup::default()),
                    args,
                    prim,
                })
            },
            [io(dst), id(opcode), tup_val(attrs), io(args), resource(prim)] => {
                let comp = CompOp::from_str(&opcode);
                Instr::from(InstrComp {
                    op: comp.unwrap(),
                    dst,
                    attrs,
                    args,
                    prim,
                })
            },
            [] => panic!(format!("Error: ~~~{}~~~ is not valid instruction", instr))
        ))
    }

    fn body(input: Node) -> Result<Vec<Instr>> {
        Ok(match_nodes!(
            input.into_children();
            [instr(instr)..] => instr.collect(),
        ))
    }

    fn def(input: Node) -> Result<Def> {
        Ok(match_nodes!(
            input.into_children();
            [body(body)] => Def {
                sig: Sig::default(),
                body,
            },
        ))
    }

    fn file(input: Node) -> Result<Def> {
        Ok(match_nodes!(
            input.into_children();
            [def(def), _] => def,
        ))
    }
}

// pub fn parse(input_str: &str) -> Prog {
pub fn parse(input_str: &str) {
    let inputs = ILParser::parse(Rule::file, input_str).expect("Error: parsing input");
    let input = inputs.single().expect("Error: parsing root");
    let prog = ILParser::file(input).expect("Error: parsing file");
    println!("{:?}", prog);
}

// pub fn parse_from_file<P: AsRef<Path>>(path: P) -> Prog {
pub fn parse_from_file<P: AsRef<Path>>(path: P) {
    let content = read_to_string(path);
    parse(&content)
}
