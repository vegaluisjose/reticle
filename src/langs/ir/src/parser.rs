use crate::ast::*;
use crate::errors::Error;
use crate::infer::infer_type_try_from_prog;
use pest_consume::Error as PestError;
use pest_consume::Parser as PestParser;
use pest_consume::match_nodes;
use std::path::Path;
use std::str::FromStr;
use utils::file::read_to_string;

pub type ParseResult<T> = std::result::Result<T, PestError<Rule>>;
type Node<'i> = pest_consume::Node<'i, Rule, ()>;

const _GRAMMAR: &str = include_str!("syntax.pest");

#[derive(PestParser)]
#[grammar = "syntax.pest"]
pub struct Parser;

#[pest_consume::parser]
impl Parser {
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

    fn op_prim(input: Node) -> ParseResult<OpPrim> {
        let op = OpPrim::from_str(input.as_str());
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

    fn instr(input: Node) -> ParseResult<Instr> {
        Ok(match_nodes!(
            input.into_children();
            [io(dst), op_prim(op), io(arg)] => Instr::from(InstrPrim {
                op,
                dst,
                attr: Expr::default(),
                arg,
                prim: Prim::Any,
            }),
            [io(dst), op_prim(op), tup_val(attr), io(arg)] => Instr::from(InstrPrim {
                op,
                dst,
                attr: Expr::from(attr),
                arg,
                prim: Prim::Any,
            }),
            [io(dst), op_prim(op), io(arg), prim(prim)] => Instr::from(InstrPrim {
                op,
                dst,
                attr: Expr::default(),
                arg,
                prim,
            }),
            [io(dst), op_prim(op), tup_val(attr), io(arg), prim(prim)] => Instr::from(InstrPrim {
                op,
                dst,
                attr: Expr::from(attr),
                arg,
                prim,
            }),
            [io(dst), op_wire(op), tup_val(attr)] => Instr::from(InstrWire {
                op,
                dst,
                attr: Expr::from(attr),
                arg: Expr::default(),
            }),
            [io(dst), op_wire(op), io(arg)] => Instr::from(InstrWire {
                op,
                dst,
                attr: Expr::default(),
                arg,
            }),
            [io(dst), op_wire(op), tup_val(attr), io(arg)] => Instr::from(InstrWire {
                op,
                dst,
                attr: Expr::from(attr),
                arg,
            }),
            [io(dst), op_call(op)] => Instr::from(InstrCall {
                op,
                dst,
                arg: Expr::default(),
            }),
            [io(dst), op_call(op), io(arg)] => Instr::from(InstrCall {
                op,
                dst,
                arg,
            }),
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

impl Parser {
    pub fn parse_from_str(input_str: &str) -> Result<Prog, Error> {
        let inputs = Parser::parse(Rule::file, input_str)?;
        let input = inputs.single()?;
        let prog = Parser::file(input)?;
        Ok(infer_type_try_from_prog(prog)?)
    }
    pub fn parse_from_file<P: AsRef<Path>>(path: P) -> Result<Prog, Error> {
        let content = read_to_string(path);
        Parser::parse_from_str(&content)
    }
}
