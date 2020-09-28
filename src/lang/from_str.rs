use crate::lang::ast::*;
use regex::Regex;
use std::rc::Rc;
use std::str::FromStr;

impl FromStr for Ty {
    type Err = String;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let re_uint = Regex::new(r"^u([[:alnum:]]+)$").unwrap();
        let re_sint = Regex::new(r"^i([[:alnum:]]+)$").unwrap();
        let re_uvec = Regex::new(r"^u([[:alnum:]]+)<([[:alnum:]]+)>$").unwrap();
        let re_svec = Regex::new(r"^i([[:alnum:]]+)<([[:alnum:]]+)>$").unwrap();
        let mut ty = Err(format!("Error: {} is not valid type", input));
        let caps;
        if input == "bool" {
            ty = Ok(Ty::Bool);
        } else if re_uint.is_match(input) {
            caps = re_uint.captures(input).unwrap();
            if let Some(w) = caps.get(1) {
                let width = w.as_str().parse::<u64>().unwrap();
                assert!(width > 0, "Error: width must be greater than zero");
                ty = Ok(Ty::UInt(width));
            }
        } else if re_sint.is_match(input) {
            caps = re_sint.captures(input).unwrap();
            if let Some(w) = caps.get(1) {
                let width = w.as_str().parse::<u64>().unwrap();
                assert!(width > 1, "Error: width must be greater than one");
                ty = Ok(Ty::SInt(width));
            }
        } else if re_uvec.is_match(input) {
            caps = re_uvec.captures(input).unwrap();
            if let Some(w) = caps.get(1) {
                if let Some(l) = caps.get(2) {
                    let width = w.as_str().parse::<u64>().unwrap();
                    let len = l.as_str().parse::<u64>().unwrap();
                    assert!(width > 0, "Error: width must be greater than zero");
                    assert!(len > 0, "Error: length must be greater than zero");
                    ty = Ok(Ty::Vector(Rc::new(Ty::UInt(width)), len));
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
                    ty = Ok(Ty::Vector(Rc::new(Ty::SInt(width)), len));
                }
            }
        }
        ty
    }
}

impl FromStr for PrimOp {
    type Err = String;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "reg" => Ok(PrimOp::Reg),
            "add" => Ok(PrimOp::Add),
            "sub" => Ok(PrimOp::Sub),
            "mul" => Ok(PrimOp::Mul),
            "not" => Ok(PrimOp::Not),
            "and" => Ok(PrimOp::And),
            "nand" => Ok(PrimOp::Nand),
            "or" => Ok(PrimOp::Or),
            "nor" => Ok(PrimOp::Nor),
            "xor" => Ok(PrimOp::Xor),
            "xnor" => Ok(PrimOp::Xnor),
            "mux" => Ok(PrimOp::Mux),
            "eq" => Ok(PrimOp::Equal),
            "neq" => Ok(PrimOp::NotEqual),
            "gt" => Ok(PrimOp::Gt),
            "lt" => Ok(PrimOp::Lt),
            "ge" => Ok(PrimOp::Ge),
            "le" => Ok(PrimOp::Le),
            _ => Err(format!("Error: {} is not valid primitive operation", input)),
        }
    }
}

impl FromStr for StdOp {
    type Err = String;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "id" => Ok(StdOp::Identity),
            "const" => Ok(StdOp::Const),
            "shl" => Ok(StdOp::ShiftLeft),
            "shr" => Ok(StdOp::ShiftRight),
            _ => Err(format!("Error: {} is not valid standard operation", input)),
        }
    }
}

impl FromStr for Loc {
    type Err = String;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "??" => Ok(Loc::Hole),
            "lut" => Ok(Loc::Lut),
            "lum" => Ok(Loc::Lum),
            "dsp" => Ok(Loc::Dsp),
            "ram" => Ok(Loc::Ram),
            _ => Err(format!("Error: {} is not valid location", input)),
        }
    }
}
