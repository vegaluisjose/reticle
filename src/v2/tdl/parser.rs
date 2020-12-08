use crate::util::file::read_to_string;
use crate::v2::tdl::ast::*;
use pest_consume::{match_nodes, Error, Parser};
use std::path::Path;
use std::str::FromStr;

pub type Result<T> = std::result::Result<T, Error<Rule>>;
type Node<'i> = pest_consume::Node<'i, Rule, ()>;

const _GRAMMAR: &str = include_str!("syntax.pest");

#[derive(Parser)]
#[grammar = "v2/tdl/syntax.pest"]
pub struct TDLParser;

#[pest_consume::parser]
impl TDLParser {
    fn EOI(_input: Node) -> Result<()> {
        Ok(())
    }

    fn id(input: Node) -> Result<Id> {
        Ok(input.as_str().to_string())
    }

    fn val(input: Node) -> Result<Expr> {
        let val = input.as_str().parse::<i64>();
        match val {
            Ok(v) => Ok(Expr::Val(v)),
            Err(_) => panic!("Error: parsing {} as i64", input.as_str()),
        }
    }

    fn cost(input: Node) -> Result<u64> {
        let val = input.as_str().parse::<u64>();
        match val {
            Ok(v) => Ok(v),
            Err(_) => panic!("Error: parsing {} as u64", input.as_str()),
        }
    }

    fn ty(input: Node) -> Result<Ty> {
        let ty = Ty::from_str(input.as_str());
        match ty {
            Ok(t) => Ok(t),
            Err(m) => panic!("{}", m),
        }
    }

    fn prim(input: Node) -> Result<Prim> {
        let prim = Prim::from_str(input.as_str());
        match prim {
            Ok(p) => Ok(p),
            Err(m) => panic!("{}", m),
        }
    }

    fn var(input: Node) -> Result<Expr> {
        Ok(match_nodes!(
            input.into_children();
            [id(id), ty(ty)] => Expr::Var(id, ty),
            [id(id)] => Expr::Var(id, Ty::Any),
        ))
    }

    fn tup_var(input: Node) -> Result<Expr> {
        Ok(match_nodes!(
            input.into_children();
            [var(vars)..] => Expr::from(ExprTup{ expr: vars.collect()}),
        ))
    }

    fn tup_val(input: Node) -> Result<Expr> {
        Ok(match_nodes!(
            input.into_children();
            [val(vals)..] => Expr::from(ExprTup{ expr: vals.collect()}),
        ))
    }

    fn io(input: Node) -> Result<Expr> {
        Ok(match_nodes!(
            input.into_children();
            [var(var)] => var,
            [tup_var(tup)] => tup,
        ))
    }

    fn instr(input: Node) -> Result<Instr> {
        let instr = input.as_str().to_string();
        Ok(match_nodes!(
            input.into_children();
            [io(dst), id(opcode), tup_val(attr)] => {
                let wop = WireOp::from_str(&opcode);
                let cop = CompOp::from_str(&opcode);
                match (wop, cop) {
                    (Ok(op), Err(_)) => Instr::from(
                        InstrWire {
                            op,
                            dst,
                            attr,
                            arg: Expr::default(),
                        }
                    ),
                    (_, _) => panic!(format!("Error: ~~~{}~~~ is not valid instruction", instr))
                }
            },
            [io(dst), id(opcode), io(arg)] => {
                let wop = WireOp::from_str(&opcode);
                let cop = CompOp::from_str(&opcode);
                match (wop, cop) {
                    (Ok(op), Err(_)) => Instr::from(
                        InstrWire {
                            op,
                            dst,
                            attr: Expr::default(),
                            arg,
                        }
                    ),
                    (Err(_), Ok(op)) => Instr::from(
                        InstrComp {
                            op,
                            dst,
                            attr: Expr::default(),
                            arg,
                            prim: Prim::Any,
                        }
                    ),
                    (_, _) => panic!(format!("Error: ~~~{}~~~ is not valid instruction", instr))
                }
            },
            [io(dst), id(opcode), tup_val(attr), io(arg)] => {
                let wop = WireOp::from_str(&opcode);
                let cop = CompOp::from_str(&opcode);
                match (wop, cop) {
                    (Ok(op), Err(_)) => Instr::from(
                        InstrWire {
                            op,
                            dst,
                            attr,
                            arg,
                        }
                    ),
                    (Err(_), Ok(op)) => Instr::from(
                        InstrComp {
                            op,
                            dst,
                            attr,
                            arg,
                            prim: Prim::Any,
                        }
                    ),
                    (_, _) => panic!(format!("Error: ~~~{}~~~ is not valid instruction", instr))
                }
            },
            [io(dst), id(opcode), io(arg), prim(prim)] => {
                let cop = CompOp::from_str(&opcode);
                match cop {
                    Ok(op) => Instr::from(InstrComp {
                        op,
                        dst,
                        attr: Expr::default(),
                        arg,
                        prim,
                    }),
                    Err(_) => panic!(format!("Error: ~~~{}~~~ is not valid instruction", instr)),
                }
            },
            [io(dst), id(opcode), tup_val(attr), io(arg), prim(prim)] => {
                let cop = CompOp::from_str(&opcode);
                match cop {
                    Ok(op) => Instr::from(InstrComp {
                        op,
                        dst,
                        attr,
                        arg,
                        prim,
                    }),
                    Err(_) => panic!(format!("Error: ~~~{}~~~ is not valid instruction", instr)),
                }
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

    fn sig(input: Node) -> Result<Sig> {
        Ok(match_nodes!(
            input.into_children();
            [id(id), prim(prim), cost(area), cost(lat), io(input), io(output)] => Sig {
                id,
                prim,
                area,
                lat,
                input,
                output,
            },
        ))
    }

    fn def(input: Node) -> Result<Def> {
        Ok(match_nodes!(
            input.into_children();
            [sig(sig), body(body)] => Def {
                sig,
                body,
            },
        ))
    }

    fn desc(input: Node) -> Result<Desc> {
        Ok(match_nodes!(
            input.into_children();
            [def(def)..] => {
                let mut desc = Desc::default();
                let defs: Vec<Def> = def.collect();
                for d in defs {
                    desc.insert(&d.id(), d.clone());
                }
                desc
            }
        ))
    }

    fn file(input: Node) -> Result<Desc> {
        Ok(match_nodes!(
            input.into_children();
            [desc(desc), _] => desc,
        ))
    }
}

impl TDLParser {
    pub fn parse_from_str(input_str: &str) -> Result<Desc> {
        let inputs = TDLParser::parse(Rule::file, input_str)?;
        let input = inputs.single()?;
        Ok(TDLParser::file(input)?)
    }
    pub fn parse_from_file<P: AsRef<Path>>(path: P) -> Result<Desc> {
        let content = read_to_string(path);
        TDLParser::parse_from_str(&content)
    }
}
