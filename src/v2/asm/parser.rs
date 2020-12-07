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
pub struct AsmParser;

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

impl AsmParser {
    pub fn parse_from_str(input_str: &str) -> Result<ExprCoord> {
        let inputs = AsmParser::parse(Rule::file, input_str)?;
        let input = inputs.single()?;
        Ok(AsmParser::file(input)?)
    }
    pub fn parse_from_file<P: AsRef<Path>>(path: P) -> Result<ExprCoord> {
        let content = read_to_string(path);
        AsmParser::parse_from_str(&content)
    }
}
