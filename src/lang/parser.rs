use crate::lang::ast::*;
use crate::lang::check::Check;
use crate::lang::infer::infer_expr_types;
use crate::util::file::read_to_string;
use pest_consume::{match_nodes, Error, Parser};
use std::path::Path;
use std::str::FromStr;

pub type Result<T> = std::result::Result<T, Error<Rule>>;
type Node<'i> = pest_consume::Node<'i, Rule, ()>;

const _GRAMMAR: &str = include_str!("grammar.pest");

#[derive(Parser)]
#[grammar = "lang/grammar.pest"]
struct ReticleParser;

#[pest_consume::parser]
impl ReticleParser {
    fn EOI(_input: Node) -> Result<()> {
        Ok(())
    }

    fn identifier(input: Node) -> Result<Id> {
        Ok(input.as_str().to_string())
    }

    fn number(input: Node) -> Result<i64> {
        Ok(input.as_str().parse::<i64>().unwrap())
    }

    fn ty(input: Node) -> Result<Ty> {
        Ok(Ty::from_str(input.as_str()).unwrap())
    }

    fn primop(input: Node) -> Result<PrimOp> {
        Ok(PrimOp::from_str(input.as_str()).unwrap())
    }

    fn stdop(input: Node) -> Result<StdOp> {
        Ok(StdOp::from_str(input.as_str()).unwrap())
    }

    fn loc(input: Node) -> Result<Loc> {
        Ok(Loc::from_str(input.as_str()).unwrap())
    }

    fn expr(input: Node) -> Result<Expr> {
        Ok(match_nodes!(
            input.into_children();
            [identifier(n), ty(ty)] => Expr::new_ref(&n, ty),
            [identifier(n)] => Expr::new_ref(&n, Ty::Hole),
            [number(n)] => Expr::new_int(n),
        ))
    }

    fn attrs(input: Node) -> Result<Vec<Expr>> {
        Ok(match_nodes!(
            input.into_children();
            [expr(expr)..] => expr.collect()
        ))
    }

    fn params(input: Node) -> Result<Vec<Expr>> {
        Ok(match_nodes!(
            input.into_children();
            [expr(expr)..] => expr.collect()
        ))
    }

    fn instr(input: Node) -> Result<Instr> {
        Ok(match_nodes!(
            input.into_children();
            [expr(dst), stdop(op), params(params)] => Instr::from(
                InstrStd {
                    op,
                    dst,
                    attrs: Vec::new(),
                    params,
                }),
            [expr(dst), stdop(op), attrs(attrs)] => Instr::from(
                InstrStd {
                    op,
                    dst,
                    attrs,
                    params: Vec::new(),
                }),
            [expr(dst), stdop(op), attrs(attrs), params(params)] => Instr::from(
                InstrStd {
                    op,
                    dst,
                    attrs,
                    params,
                }),
            [expr(dst), primop(op), params(params), loc(loc)] => Instr::from(
                InstrPrim {
                    op,
                    dst,
                    attrs: Vec::new(),
                    params,
                    loc,
                }),
            [expr(dst), primop(op), attrs(attrs), loc(loc)] => Instr::from(
                InstrPrim {
                    op,
                    dst,
                    attrs,
                    params: Vec::new(),
                    loc,
                }),
            [expr(dst), primop(op), attrs(attrs), params(params), loc(loc)] => Instr::from(
                InstrPrim {
                    op,
                    dst,
                    attrs,
                    params,
                    loc,
                }),
            [expr(dst), primop(op), params(params)] => Instr::from(
                InstrPrim {
                    op,
                    dst,
                    attrs: Vec::new(),
                    params,
                    loc: Loc::Hole,
                }),
            [expr(dst), primop(op), attrs(attrs)] => Instr::from(
                InstrPrim {
                    op,
                    dst,
                    attrs,
                    params: Vec::new(),
                    loc: Loc::Hole,
                }),
            [expr(dst), primop(op), attrs(attrs), params(params)] => Instr::from(
                InstrPrim {
                    op,
                    dst,
                    attrs,
                    params,
                    loc: Loc::Hole,
                }),
        ))
    }

    fn instrs(input: Node) -> Result<Vec<Instr>> {
        Ok(match_nodes!(
            input.into_children();
            [instr(instr)..] => instr.collect()
        ))
    }

    fn input(input: Node) -> Result<Port> {
        Ok(match_nodes!(
            input.into_children();
            [expr(expr)] => Port::Input(expr)
        ))
    }

    fn inputs(input: Node) -> Result<Vec<Port>> {
        Ok(match_nodes!(
            input.into_children();
            [input(input)..] => input.collect()
        ))
    }

    fn output(input: Node) -> Result<Port> {
        Ok(match_nodes!(
            input.into_children();
            [expr(expr)] => Port::Output(expr)
        ))
    }

    fn outputs(input: Node) -> Result<Vec<Port>> {
        Ok(match_nodes!(
            input.into_children();
            [output(output)..] => output.collect()
        ))
    }

    fn def(input: Node) -> Result<Def> {
        Ok(match_nodes!(
            input.into_children();
            [identifier(id), inputs(inputs), outputs(outputs), instrs(body)] => Def {
                sig: Signature {
                    id,
                    inputs,
                    outputs,
                },
                body,
            }
        ))
    }

    fn file(input: Node) -> Result<Prog> {
        Ok(match_nodes!(
            input.into_children();
            [def(def).., _] => Prog {
                defs: def.collect(),
            }
        ))
    }
}

pub fn parse(input_str: &str) -> Prog {
    let inputs = ReticleParser::parse(Rule::file, input_str).expect("Error: parsing inputs");
    let input = inputs.single().expect("Error: parsing root");
    let prog = ReticleParser::file(input).expect("Error: parsing file");
    let prog_infer = infer_expr_types(&prog);
    prog_infer.check();
    prog_infer
}

pub fn parse_from_file<P: AsRef<Path>>(path: P) -> Prog {
    let content = read_to_string(path);
    parse(&content)
}