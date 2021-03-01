use reticle::asm::ast as asm;
use reticle::asm::parser::AsmParser;
use reticle::util::errors::Error;
use std::collections::HashMap;

type Map = HashMap<String, asm::InstrAsm>;
type Pair = HashMap<String, String>;

fn create_map_and_pair(prog: &asm::Prog) -> Result<(Map, Pair), Error> {
    let mut map = Map::new();
    let mut pair = Pair::new();
    for instr in prog.body() {
        match instr {
            asm::Instr::Asm(instr) if instr.op().name().as_str() == "dsp_muladd_i8_ra_rb_rm_rp" => {
                let dst = instr.dst().get_term(0)?;
                let arg = instr.arg().get_term(2)?;
                let dst_id = dst.get_id()?;
                let arg_id = arg.get_id()?;
                map.insert(dst_id.clone(), instr.clone());
                pair.insert(arg_id, dst_id);
            }
            _ => (),
        }
    }
    let keys: Vec<String> = pair.keys().cloned().collect();
    for k in keys {
        if !map.contains_key(&k) {
            pair.remove(&k);
        }
    }
    Ok((map, pair))
}

fn find_tail(pair: &Pair) -> String {
    let keys: Vec<String> = pair.keys().cloned().collect();
    let mut tail = keys[0].clone();
    let mut prev = String::new();
    while tail != prev {
        prev = tail.clone();
        for (k, v) in pair {
            if v == &tail {
                tail = k.clone();
            }
        }
    }
    tail
}

fn find_cascade(pair: &Pair, tail: &str) -> (Vec<String>, Pair) {
    let mut stack: Vec<String> = Vec::new();
    stack.push(tail.to_string());
    let mut cascade: Vec<String> = Vec::new();
    let mut pair = pair.clone();
    while let Some(cur) = stack.pop() {
        cascade.push(cur.clone());
        if let Some(next) = pair.get(&cur) {
            stack.push(next.clone());
            pair.remove(&cur);
        }
    }
    (cascade, pair)
}

fn find_all_cascade(pair: &Pair) -> Vec<Vec<String>> {
    let mut pair = pair.clone();
    let mut res: Vec<Vec<String>> = Vec::new();
    while !pair.is_empty() {
        let tail = find_tail(&pair);
        let (c, p) = find_cascade(&pair, &tail);
        pair = p;
        res.push(c);
    }
    res
}

fn main() -> Result<(), Error> {
    let prog = AsmParser::parse_from_file("examples/asm/tdot.asm")?;
    let (_, pair) = create_map_and_pair(&prog)?;
    let x = find_all_cascade(&pair);
    for i in x {
        println!("{:?}", i);
    }
    Ok(())
}
