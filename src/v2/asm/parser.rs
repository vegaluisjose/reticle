use crate::util::file::read_to_string;
use crate::v2::asm::ast::*;
use pest_consume::{match_nodes, Error, Parser};
use std::path::Path;
use std::str::FromStr;

pub type Result<T> = std::result::Result<T, Error<Rule>>;
type Node<'i> = pest_consume::Node<'i, Rule, ()>;

const _GRAMMAR: &str = include_str!("grammar.pest");

#[derive(Parser)]
#[grammar = "v2/asm/grammar.pest"]
struct AsmParser;

#[pest_consume::parser]
impl AsmParser {
    fn EOI(_input: Node) -> Result<()> {
        Ok(())
    }

    fn id(input: Node) -> Result<Id> {
        Ok(input.as_str().to_string())
    }

    fn expr_coord(input: Node) -> Result<ExprCoord> {
        let expr = ExprCoord::from_str(input.as_str());
        match expr {
            Ok(e) => Ok(e),
            Err(m) => panic!("{}", m),
        }
    }

    fn file(input: Node) -> Result<ExprCoord> {
        Ok(match_nodes!(
            input.into_children();
            [expr_coord(e), _] => e,
        ))
    }
}

// pub fn parse(input_str: &str) -> Prog {
pub fn parse(input_str: &str) {
    let inputs = AsmParser::parse(Rule::file, input_str).expect("Error: parsing input");
    let input = inputs.single().expect("Error: parsing root");
    let e = AsmParser::file(input).expect("Error: parsing file");
    println!("{:?}", e);
}

// pub fn parse_from_file<P: AsRef<Path>>(path: P) -> Prog {
pub fn parse_from_file<P: AsRef<Path>>(path: P) {
    let content = read_to_string(path);
    parse(&content)
}
