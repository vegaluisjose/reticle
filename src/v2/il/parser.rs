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

    fn val(input: Node) -> Result<Expr> {
        let val = input.as_str().parse::<i64>().unwrap();
        Ok(Expr::Val(val))
    }

    fn name(input: Node) -> Result<Expr> {
        Ok(match_nodes!(
            input.into_children();
            [id(id), ty(ty)] => Expr::Name(id, ty)
        ))
    }

    fn name_or_val(input: Node) -> Result<Expr> {
        Ok(match_nodes!(
            input.into_children();
            [name(name)] => name,
            [val(val)] => val,
        ))
    }

    fn expr_tup(input: Node) -> Result<Expr> {
        Ok(match_nodes!(
            input.into_children();
            [name_or_val(exprs)..] => Expr::from(ExprTup{ exprs: exprs.collect()}),
        ))
    }

    fn file(input: Node) -> Result<Expr> {
        Ok(match_nodes!(
            input.into_children();
            [expr_tup(expr), _] => expr,
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
