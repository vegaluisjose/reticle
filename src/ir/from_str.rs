use crate::ir::ast::*;
use crate::util::errors::Error;
use regex::Regex;
use std::rc::Rc;
use std::str::FromStr;

impl FromStr for Ty {
    type Err = Error;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let re_uint = Regex::new(r"^u([[:alnum:]]+)$").unwrap();
        let re_sint = Regex::new(r"^i([[:alnum:]]+)$").unwrap();
        let re_uvec = Regex::new(r"^u([[:alnum:]]+)<([[:alnum:]]+)>$").unwrap();
        let re_svec = Regex::new(r"^i([[:alnum:]]+)<([[:alnum:]]+)>$").unwrap();
        let err = format!("Error: {} is not valid type", input);
        if input == "bool" {
            Ok(Ty::Bool)
        } else if re_uint.is_match(input) {
            let caps = re_uint.captures(input).unwrap();
            if let Some(w) = caps.get(1) {
                let width = w.as_str().parse::<u64>().unwrap();
                assert!(width > 0, "Error: width must be greater than zero");
                Ok(Ty::UInt(width))
            } else {
                Err(Error::new_parse_error(&err))
            }
        } else if re_sint.is_match(input) {
            let caps = re_sint.captures(input).unwrap();
            if let Some(w) = caps.get(1) {
                let width = w.as_str().parse::<u64>().unwrap();
                assert!(width > 1, "Error: width must be greater than one");
                Ok(Ty::SInt(width))
            } else {
                Err(Error::new_parse_error(&err))
            }
        } else if re_uvec.is_match(input) {
            let caps = re_uvec.captures(input).unwrap();
            if let Some(w) = caps.get(1) {
                if let Some(l) = caps.get(2) {
                    let width = w.as_str().parse::<u64>().unwrap();
                    let len = l.as_str().parse::<u64>().unwrap();
                    assert!(width > 0, "Error: width must be greater than zero");
                    assert!(len > 0, "Error: length must be greater than zero");
                    Ok(Ty::Vector(Rc::new(Ty::UInt(width)), len))
                } else {
                    Err(Error::new_parse_error(&err))
                }
            } else {
                Err(Error::new_parse_error(&err))
            }
        } else if re_svec.is_match(input) {
            let caps = re_svec.captures(input).unwrap();
            if let Some(w) = caps.get(1) {
                if let Some(l) = caps.get(2) {
                    let width = w.as_str().parse::<u64>().unwrap();
                    let len = l.as_str().parse::<u64>().unwrap();
                    assert!(width > 1, "Error: width must be greater than one");
                    assert!(len > 0, "Error: length must be greater than zero");
                    Ok(Ty::Vector(Rc::new(Ty::SInt(width)), len))
                } else {
                    Err(Error::new_parse_error(&err))
                }
            } else {
                Err(Error::new_parse_error(&err))
            }
        } else {
            Err(Error::new_parse_error(&err))
        }
    }
}

impl FromStr for OpComp {
    type Err = Error;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let err = format!("Error: {} is not valid compute operation", input);
        match input {
            "reg" => Ok(OpComp::Reg),
            "add" => Ok(OpComp::Add),
            "sub" => Ok(OpComp::Sub),
            "mul" => Ok(OpComp::Mul),
            "not" => Ok(OpComp::Not),
            "and" => Ok(OpComp::And),
            "or" => Ok(OpComp::Or),
            "xor" => Ok(OpComp::Xor),
            "mux" => Ok(OpComp::Mux),
            "eq" => Ok(OpComp::Eql),
            "neq" => Ok(OpComp::Neql),
            "gt" => Ok(OpComp::Gt),
            "lt" => Ok(OpComp::Lt),
            "ge" => Ok(OpComp::Ge),
            "le" => Ok(OpComp::Le),
            _ => Err(Error::new_parse_error(&err)),
        }
    }
}

impl FromStr for OpWire {
    type Err = Error;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let err = format!("Error: {} is not valid wire operation", input);
        match input {
            "id" => Ok(OpWire::Id),
            "inp" => Ok(OpWire::Inp),
            "const" => Ok(OpWire::Con),
            "sll" => Ok(OpWire::Sll),
            "srl" => Ok(OpWire::Srl),
            "sra" => Ok(OpWire::Sra),
            _ => Err(Error::new_parse_error(&err)),
        }
    }
}

impl FromStr for CallOp {
    type Err = Error;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(CallOp::Op(input.to_string()))
    }
}

impl FromStr for Prim {
    type Err = Error;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let err = format!("Error: {} is not valid primitive", input);
        match input {
            "??" => Ok(Prim::Any),
            "lut" => Ok(Prim::Lut),
            "dsp" => Ok(Prim::Dsp),
            _ => Err(Error::new_parse_error(&err)),
        }
    }
}
