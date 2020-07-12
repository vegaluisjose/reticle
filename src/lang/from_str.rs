use crate::lang::ast::*;
use regex::Regex;
use std::rc::Rc;
use std::str::FromStr;

impl FromStr for DataType {
    type Err = ();
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let re_bool = Regex::new(r"bool$").unwrap();
        let re_uint = Regex::new(r"^u([[:alnum:]]+)$").unwrap();
        let re_sint = Regex::new(r"^i([[:alnum:]]+)$").unwrap();
        let re_uvec = Regex::new(r"^u([[:alnum:]]+)<([[:alnum:]]+)>$").unwrap();
        let re_svec = Regex::new(r"^i([[:alnum:]]+)<([[:alnum:]]+)>$").unwrap();
        let mut dtype = Err(());
        let caps;
        if re_bool.is_match(input) {
            dtype = Ok(DataType::Bool);
        } else if re_uint.is_match(input) {
            caps = re_uint.captures(input).unwrap();
            if let Some(w) = caps.get(1) {
                let width = w.as_str().parse::<u64>().unwrap();
                assert!(width > 0, "Error: width must be greater than zero");
                dtype = Ok(DataType::UInt(width));
            }
        } else if re_sint.is_match(input) {
            caps = re_sint.captures(input).unwrap();
            if let Some(w) = caps.get(1) {
                let width = w.as_str().parse::<u64>().unwrap();
                assert!(width > 1, "Error: width must be greater than one");
                dtype = Ok(DataType::SInt(width));
            }
        } else if re_uvec.is_match(input) {
            caps = re_uvec.captures(input).unwrap();
            if let Some(w) = caps.get(1) {
                if let Some(l) = caps.get(2) {
                    let width = w.as_str().parse::<u64>().unwrap();
                    let len = l.as_str().parse::<u64>().unwrap();
                    assert!(width > 0, "Error: width must be greater than zero");
                    assert!(len > 0, "Error: length must be greater than zero");
                    dtype = Ok(DataType::Vector(Rc::new(DataType::UInt(width)), len));
                }
            }
        } else if re_svec.is_match(input) {
            caps = re_svec.captures(input).unwrap();
            if let Some(w) = caps.get(1) {
                if let Some(l) = caps.get(2) {
                    let width = w.as_str().parse::<u64>().unwrap();
                    let len = l.as_str().parse::<u64>().unwrap();
                    assert!(width > 1, "Error: width must be greater than one");
                    assert!(len > 0, "Error: length must be greater than zero");
                    dtype = Ok(DataType::Vector(Rc::new(DataType::SInt(width)), len));
                }
            }
        }
        dtype
    }
}

impl FromStr for PlacedOp {
    type Err = ();
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input.as_ref() {
            "reg" => Ok(PlacedOp::Reg),
            "add" => Ok(PlacedOp::Add),
            "sub" => Ok(PlacedOp::Sub),
            "mul" => Ok(PlacedOp::Mul),
            _ => panic!("WIP"),
        }
    }
}