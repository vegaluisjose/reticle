use crate::lang::ast::*;
use crate::util::file::read_to_string;
use pest_consume::{match_nodes, Error, Parser};
use std::path::Path;
use std::str::FromStr;

pub type Result<T> = std::result::Result<T, Error<Rule>>;
type Node<'i> = pest_consume::Node<'i, Rule, ()>;

// trick to make cargo build when grammar changes, taken from
// https://github.com/cucapra/futil/blob/master/calyx/src/frontend/library_syntax.rs
const _GRAMMAR: &str = include_str!("grammar.pest");

#[derive(Parser)]
#[grammar = "frontend/grammar.pest"]
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

    fn expr(input: Node) -> Result<Expr> {
        Ok(match_nodes!(
            input.into_children();
            [identifier(n)] => Expr::Ref(n, Ty::Hole),
        ))
    }

    fn primop(input: Node) -> Result<PrimOp> {
        Ok(PrimOp::from_str(input.as_str()).unwrap())
    }

    fn prim(input: Node) -> Result<Instr> {
        Ok(match_nodes!(
            input.into_children();
            [identifier(id), primop(op)] => Instr::Prim {
                id,
                ty: Ty::Hole,
                op,
                attrs: Vec::new(),
                params: Vec::new(),
                loc: Loc::Hole,
            },
        ))
    }

    // fn file(input: Node) -> Result<Expr> {
    //     Ok(match_nodes!(
    //         input.into_children();
    //         [expr(e)] => e,
    //     ))
    // }
}

pub fn parse(input_str: &str) -> Result<Instr> {
    let inputs = ReticleParser::parse(Rule::prim, input_str)?;
    let input = inputs.single()?;
    ReticleParser::prim(input)
}

pub fn parse_from_file<P: AsRef<Path>>(path: P) -> Result<Instr> {
    let content = read_to_string(path);
    parse(&content)
}
