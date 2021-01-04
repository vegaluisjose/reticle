use crate::ir::ast::*;
use crate::util::errors::Error;
use regex::Regex;
use std::rc::Rc;
use std::str::FromStr;

const RE_BOOL: &str = r"^[\s\t]*bool+[\s\t]*$";
const RE_UINT: &str = r"^[\s\t]*u[[:digit:]]+[\s\t]*$";
const RE_SINT: &str = r"^[\s\t]*i[[:digit:]]+[\s\t]*$";
const RE_UVEC: &str = r"^[\s\t]*u[[:digit:]]+<[[:digit:]]+>[\s\t]*$";
const RE_SVEC: &str = r"^[\s\t]*i[[:digit:]]+<[[:digit:]]+>[\s\t]*$";
const RE_LENGTH: &str = r"^[\s\t]*[ui][[:digit:]]+<([[:digit:]]+)>[\s\t]*$";
const RE_WIDTH: &str = r"^[\s\t]*[ui]([[:digit:]]+)[<[[:digit:]]+>]*[\s\t]*$";

fn is_bool(input: &str) -> bool {
    lazy_static::lazy_static! {
        static ref RE: Regex = Regex::new(RE_BOOL).unwrap();
    }
    RE.is_match(input)
}

fn is_uint(input: &str) -> bool {
    lazy_static::lazy_static! {
        static ref RE: Regex = Regex::new(RE_UINT).unwrap();
    }
    RE.is_match(input)
}

fn is_sint(input: &str) -> bool {
    lazy_static::lazy_static! {
        static ref RE: Regex = Regex::new(RE_SINT).unwrap();
    }
    RE.is_match(input)
}

fn is_uvec(input: &str) -> bool {
    lazy_static::lazy_static! {
        static ref RE: Regex = Regex::new(RE_UVEC).unwrap();
    }
    RE.is_match(input)
}

fn is_svec(input: &str) -> bool {
    lazy_static::lazy_static! {
        static ref RE: Regex = Regex::new(RE_SVEC).unwrap();
    }
    RE.is_match(input)
}

fn width(input: &str) -> Result<u64, Error> {
    lazy_static::lazy_static! {
        static ref RE: Regex = Regex::new(RE_WIDTH).unwrap();
    }
    let caps = RE.captures(input).unwrap();
    let err = format!("{} is not valid width", input);
    if let Some(first) = caps.get(1) {
        if let Ok(width) = first.as_str().parse::<u64>() {
            Ok(width)
        } else {
            Err(Error::new_conv_error(&err))
        }
    } else {
        Err(Error::new_conv_error(&err))
    }
}

fn length(input: &str) -> Result<u64, Error> {
    lazy_static::lazy_static! {
        static ref RE: Regex = Regex::new(RE_LENGTH).unwrap();
    }
    let caps = RE.captures(input).unwrap();
    let err = format!("{} is not valid length", input);
    if let Some(first) = caps.get(1) {
        if let Ok(width) = first.as_str().parse::<u64>() {
            Ok(width)
        } else {
            Err(Error::new_conv_error(&err))
        }
    } else {
        Err(Error::new_conv_error(&err))
    }
}

impl FromStr for Ty {
    type Err = Error;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let err = format!("Error: {} is not valid type", input);
        if is_bool(input) {
            Ok(Ty::Bool)
        } else if is_uint(input) {
            Ok(Ty::UInt(width(input)?))
        } else if is_sint(input) {
            Ok(Ty::SInt(width(input)?))
        } else if is_uvec(input) {
            Ok(Ty::Vector(Rc::new(Ty::UInt(width(input)?)), length(input)?))
        } else if is_svec(input) {
            Ok(Ty::Vector(Rc::new(Ty::SInt(width(input)?)), length(input)?))
        } else {
            Err(Error::new_conv_error(&err))
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
            _ => Err(Error::new_conv_error(&err)),
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
            "ext" => Ok(OpWire::Ext),
            "cat" => Ok(OpWire::Cat),
            _ => Err(Error::new_conv_error(&err)),
        }
    }
}

impl FromStr for OpCall {
    type Err = Error;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        if OpWire::from_str(input).is_ok() {
            let err = format!(
                "Error: {} is a wire operation and cannot be a call operation",
                input
            );
            Err(Error::new_conv_error(&err))
        } else if OpComp::from_str(input).is_ok() {
            let err = format!(
                "Error: {} is a comp operation and cannot be a call operation",
                input
            );
            Err(Error::new_conv_error(&err))
        } else {
            Ok(OpCall::new(input))
        }
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
            _ => Err(Error::new_conv_error(&err)),
        }
    }
}
