use crate::ir::ast::*;
use crate::ir::infer::infer_type_try_from_prog;
use crate::util::errors::Error;
use crate::util::file::read_to_string;
use pest_consume::Error as PestError;
use pest_consume::{match_nodes, Parser};
use std::path::Path;
use std::str::FromStr;

pub type ParseResult<T> = std::result::Result<T, PestError<Rule>>;
type Node<'i> = pest_consume::Node<'i, Rule, ()>;

const _GRAMMAR: &str = include_str!("syntax.pest");

#[derive(Parser)]
#[grammar = "ir/syntax.pest"]
pub struct IRParser;

#[pest_consume::parser]
impl IRParser {
    fn EOI(_input: Node) -> ParseResult<()> {
        Ok(())
    }

    fn id(input: Node) -> ParseResult<Id> {
        Ok(input.as_str().to_string())
    }

    fn val(input: Node) -> ParseResult<ExprTerm> {
        let val = input.as_str().parse::<i64>();
        match val {
            Ok(v) => Ok(ExprTerm::Val(v)),
            Err(_) => panic!("Error: parsing {} as i64", input.as_str()),
        }
    }

    fn ty(input: Node) -> ParseResult<Ty> {
        let ty = Ty::from_str(input.as_str());
        match ty {
            Ok(t) => Ok(t),
            Err(m) => panic!("{}", m),
        }
    }

    fn prim(input: Node) -> ParseResult<Prim> {
        let prim = Prim::from_str(input.as_str());
        match prim {
            Ok(p) => Ok(p),
            Err(m) => panic!("{}", m),
        }
    }

    fn var(input: Node) -> ParseResult<ExprTerm> {
        Ok(match_nodes!(
            input.into_children();
            [id(id), ty(ty)] => ExprTerm::Var(id, ty),
            [id(id)] => ExprTerm::Var(id, Ty::Any),
        ))
    }

    fn tup_var(input: Node) -> ParseResult<ExprTup> {
        Ok(match_nodes!(
            input.into_children();
            [var(vars)..] => ExprTup{ term: vars.collect() },
        ))
    }

    fn tup_val(input: Node) -> ParseResult<ExprTup> {
        Ok(match_nodes!(
            input.into_children();
            [val(vals)..] => ExprTup{ term: vals.collect() },
        ))
    }

    fn io(input: Node) -> ParseResult<Expr> {
        Ok(match_nodes!(
            input.into_children();
            [var(var)] => Expr::from(var),
            [tup_var(tup)] => Expr::from(tup),
        ))
    }

    fn op_comp(input: Node) -> ParseResult<OpComp> {
        let op = OpComp::from_str(input.as_str());
        match op {
            Ok(t) => Ok(t),
            Err(m) => panic!("{}", m),
        }
    }

    fn op_wire(input: Node) -> ParseResult<OpWire> {
        let op = OpWire::from_str(input.as_str());
        match op {
            Ok(t) => Ok(t),
            Err(m) => panic!("{}", m),
        }
    }

    fn op_call(input: Node) -> ParseResult<OpCall> {
        let op = OpCall::from_str(input.as_str());
        match op {
            Ok(t) => Ok(t),
            Err(m) => panic!("{}", m),
        }
    }

    fn instr_comp(input: Node) -> ParseResult<InstrComp> {
        Ok(match_nodes!(
            input.into_children();
            [io(dst), op_comp(op), io(arg)] => InstrComp {
                op,
                dst,
                attr: Expr::default(),
                arg,
                prim: Prim::Any,
            },
            [io(dst), op_comp(op), tup_val(attr), io(arg)] => InstrComp {
                op,
                dst,
                attr: Expr::from(attr),
                arg,
                prim: Prim::Any,
            },
            [io(dst), op_comp(op), io(arg), prim(prim)] => InstrComp {
                op,
                dst,
                attr: Expr::default(),
                arg,
                prim,
            },
            [io(dst), op_comp(op), tup_val(attr), io(arg), prim(prim)] => InstrComp {
                op,
                dst,
                attr: Expr::from(attr),
                arg,
                prim,
            }
        ))
    }

    fn instr_wire(input: Node) -> ParseResult<InstrWire> {
        Ok(match_nodes!(
            input.into_children();
            [io(dst), op_wire(op), tup_val(attr)] => InstrWire {
                op,
                dst,
                attr: Expr::from(attr),
                arg: Expr::default(),
            },
            [io(dst), op_wire(op), io(arg)] => InstrWire {
                op,
                dst,
                attr: Expr::default(),
                arg,
            },
            [io(dst), op_wire(op), tup_val(attr), io(arg)] => InstrWire {
                op,
                dst,
                attr: Expr::from(attr),
                arg,
            }
        ))
    }

    fn instr_call(input: Node) -> ParseResult<InstrCall> {
        Ok(match_nodes!(
            input.into_children();
            [io(dst), op_call(op)] => InstrCall {
                op,
                dst,
                arg: Expr::default(),
            },
            [io(dst), op_call(op), io(arg)] => InstrCall {
                op,
                dst,
                arg,
            },
        ))
    }

    fn instr(input: Node) -> ParseResult<Instr> {
        Ok(match_nodes!(
            input.into_children();
            [instr_comp(instr)] => Instr::from(instr),
            [instr_wire(instr)] => Instr::from(instr),
            [instr_call(instr)] => Instr::from(instr),
        ))
    }

    fn body(input: Node) -> ParseResult<Vec<Instr>> {
        Ok(match_nodes!(
            input.into_children();
            [instr(instr)..] => instr.collect(),
        ))
    }

    fn sig(input: Node) -> ParseResult<Sig> {
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

    fn def(input: Node) -> ParseResult<Def> {
        Ok(match_nodes!(
            input.into_children();
            [sig(sig), body(body)] => Def {
                sig,
                body,
            },
        ))
    }

    fn prog(input: Node) -> ParseResult<Prog> {
        Ok(match_nodes!(
            input.into_children();
            [def(def)..] => {
                let mut prog = Prog::default();
                let defs: Vec<Def> = def.collect();
                for d in defs {
                    prog.insert(&d.id(), d.clone());
                }
                prog
            }
        ))
    }

    fn file(input: Node) -> ParseResult<Prog> {
        Ok(match_nodes!(
            input.into_children();
            [prog(prog), _] => prog,
        ))
    }
}

impl IRParser {
    pub fn parse_from_str(input_str: &str) -> Result<Prog, Error> {
        let inputs = IRParser::parse(Rule::file, input_str)?;
        let input = inputs.single()?;
        let prog = IRParser::file(input)?;
        Ok(infer_type_try_from_prog(prog)?)
    }
    pub fn parse_from_file<P: AsRef<Path>>(path: P) -> Result<Prog, Error> {
        let content = read_to_string(path);
        IRParser::parse_from_str(&content)
    }
}
