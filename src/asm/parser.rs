use crate::asm::ast::*;
use crate::util::file::read_to_string;
use pest_consume::{match_nodes, Error, Parser};
use std::path::Path;
use std::str::FromStr;

pub type Result<T> = std::result::Result<T, Error<Rule>>;
type Node<'i> = pest_consume::Node<'i, Rule, ()>;

const _GRAMMAR: &str = include_str!("syntax.pest");

#[derive(Parser)]
#[grammar = "asm/syntax.pest"]
pub struct AsmParser;

#[pest_consume::parser]
impl AsmParser {
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

    fn ty(input: Node) -> Result<Ty> {
        let ty = Ty::from_str(input.as_str());
        match ty {
            Ok(t) => Ok(t),
            Err(m) => panic!("{}", m),
        }
    }

    fn expr_coord(input: Node) -> Result<ExprCoord> {
        let expr = ExprCoord::from_str(input.as_str());
        match expr {
            Ok(e) => Ok(e),
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

    fn loc(input: Node) -> Result<Loc> {
        Ok(match_nodes!(
            input.into_children();
            [prim(prim)] => Loc {
                prim,
                x: ExprCoord::Any,
                y: ExprCoord::Any,
            },
            [prim(prim), expr_coord(x), expr_coord(y)] => Loc {
                prim,
                x,
                y,
            },
        ))
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
                let wop = OpWire::from_str(&opcode);
                match wop {
                    Ok(op) => Instr::from(
                        InstrWire {
                            op,
                            dst,
                            attr,
                            arg: Expr::default(),
                        }
                    ),
                    Err(_) => panic!(format!("Error: ~~~{}~~~ is not valid instruction", instr))
                }
            },
            [io(dst), id(opcode), io(arg)] => {
                let wop = OpWire::from_str(&opcode);
                match wop {
                    Ok(op) => Instr::from(
                        InstrWire {
                            op,
                            dst,
                            attr: Expr::default(),
                            arg,
                        }
                    ),
                    Err(_) => panic!(format!("Error: ~~~{}~~~ is not valid instruction", instr))
                }
            },
            [io(dst), id(opcode), tup_val(attr), io(arg)] => {
                let wop = OpWire::from_str(&opcode);
                match wop {
                    Ok(op) => Instr::from(
                        InstrWire {
                            op,
                            dst,
                            attr,
                            arg,
                        }
                    ),
                    Err(_) => panic!(format!("Error: ~~~{}~~~ is not valid instruction", instr))
                }
            },
            [io(dst), id(opcode), io(arg), loc(loc)] => {
                let aop = OpAsm::from_str(&opcode);
                match aop {
                    Ok(op) => Instr::from(
                        InstrAsm {
                            op,
                            dst,
                            attr: Expr::default(),
                            arg,
                            loc,
                            area: 0,
                            lat: 0,
                        }
                    ),
                    Err(_) => panic!(format!("Error: ~~~{}~~~ is not valid instruction", instr))
                }
            },
            [io(dst), id(opcode), tup_val(attr), io(arg), loc(loc)] => {
                let aop = OpAsm::from_str(&opcode);
                match aop {
                    Ok(op) => Instr::from(
                        InstrAsm {
                            op,
                            dst,
                            attr,
                            arg,
                            loc,
                            area: 0,
                            lat: 0,
                        }
                    ),
                    Err(_) => panic!(format!("Error: ~~~{}~~~ is not valid instruction", instr))
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
            [id(id), io(output)] => Sig {
                id,
                input: Expr::default(),
                output,
            },
            [id(id), io(input), io(output)] => Sig {
                id,
                input,
                output,
            },
        ))
    }

    fn prog(input: Node) -> Result<Prog> {
        Ok(match_nodes!(
            input.into_children();
            [sig(sig), body(body)] => Prog {
                sig,
                body,
            },
        ))
    }

    fn file(input: Node) -> Result<Prog> {
        Ok(match_nodes!(
            input.into_children();
            [prog(p), _] => p,
        ))
    }
}

impl AsmParser {
    pub fn parse_from_str(input_str: &str) -> Result<Prog> {
        let inputs = AsmParser::parse(Rule::file, input_str)?;
        let input = inputs.single()?;
        Ok(AsmParser::file(input)?)
    }
    pub fn parse_from_file<P: AsRef<Path>>(path: P) -> Result<Prog> {
        let content = read_to_string(path);
        AsmParser::parse_from_str(&content)
    }
}
