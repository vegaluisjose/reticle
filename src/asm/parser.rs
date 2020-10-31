use crate::asm::ast::*;
// use crate::lang::check::Check;
// use crate::lang::infer::infer_expr_types;
use crate::util::file::read_to_string;
use pest_consume::{match_nodes, Error, Parser};
use std::path::Path;
use std::str::FromStr;

pub type Result<T> = std::result::Result<T, Error<Rule>>;
type Node<'i> = pest_consume::Node<'i, Rule, ()>;

const _GRAMMAR: &str = include_str!("grammar.pest");

#[derive(Parser)]
#[grammar = "asm/grammar.pest"]
struct AsmParser;

#[pest_consume::parser]
impl AsmParser {
    fn EOI(_input: Node) -> Result<()> {
        Ok(())
    }

    fn id(input: Node) -> Result<Id> {
        Ok(input.as_str().to_string())
    }

    fn num(input: Node) -> Result<i64> {
        Ok(input.as_str().parse::<i64>().unwrap())
    }

    fn ty(input: Node) -> Result<Ty> {
        Ok(Ty::from_str(input.as_str()).unwrap())
    }

    fn expr(input: Node) -> Result<Expr> {
        Ok(match_nodes!(
            input.into_children();
            [id(n), ty(ty)] => Expr::new_ref(&n, ty),
            [id(n)] => Expr::new_ref(&n, Ty::Hole),
            [num(n)] => Expr::new_int(n),
        ))
    }

    fn inputs(input: Node) -> Result<Vec<Port>> {
        Ok(match_nodes!(
            input.into_children();
            [expr(expr)..] => expr.into_iter().map(|e| Port::Input(e)).collect()
        ))
    }

    fn outputs(input: Node) -> Result<Vec<Port>> {
        Ok(match_nodes!(
            input.into_children();
            [expr(expr)..] => expr.into_iter().map(|e| Port::Output(e)).collect()
        ))
    }

    fn prog(input: Node) -> Result<Prog> {
        Ok(match_nodes!(
            input.into_children();
            [id(id), inputs(inputs), outputs(outputs)] => Prog {
                sig: Signature {
                    id,
                    inputs,
                    outputs,
                },
                body: vec![],
            }
        ))
    }

    fn file(input: Node) -> Result<Prog> {
        Ok(match_nodes!(
            input.into_children();
            [prog(prog), _] => prog,
        ))
    }
}

pub fn parse(input_str: &str) -> Prog {
    let inputs = AsmParser::parse(Rule::file, input_str).expect("Error: parsing inputs");
    let input = inputs.single().expect("Error: parsing root");
    let prog = AsmParser::file(input).expect("Error: parsing file");
    prog
    // let prog_infer = infer_expr_types(&prog);
    // prog_infer.check();
    // prog_infer
}

pub fn parse_from_file<P: AsRef<Path>>(path: P) -> Prog {
    let content = read_to_string(path);
    parse(&content)
}
