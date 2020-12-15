use crate::ml::ast::*;
use crate::util::errors::Error;
use std::str::FromStr;

impl FromStr for OpLut {
    type Err = Error;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let err = format!("Error: {} is not valid lut operation", input);
        match input {
            "lut1" => Ok(OpLut::Lut1),
            "lut2" => Ok(OpLut::Lut2),
            "lut3" => Ok(OpLut::Lut3),
            "lut4" => Ok(OpLut::Lut4),
            "lut5" => Ok(OpLut::Lut5),
            "lut6" => Ok(OpLut::Lut6),
            _ => Err(Error::new_parse_error(&err)),
        }
    }
}

impl FromStr for OpReg {
    type Err = Error;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let err = format!("Error: {} is not valid reg operation", input);
        match input {
            "fdre" => Ok(OpReg::Fdre),
            "fdse" => Ok(OpReg::Fdse),
            _ => Err(Error::new_parse_error(&err)),
        }
    }
}

impl FromStr for OpDsp {
    type Err = Error;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let err = format!("Error: {} is not valid dsp operation", input);
        match input {
            "dsp_add" => Ok(OpDsp::Add),
            "dsp_muladd" => Ok(OpDsp::MulAdd),
            _ => Err(Error::new_parse_error(&err)),
        }
    }
}

impl FromStr for OpCarry {
    type Err = Error;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let err = format!("Error: {} is not valid carry operation", input);
        match input {
            "carry" => Ok(OpCarry::Carry8),
            _ => Err(Error::new_parse_error(&err)),
        }
    }
}

impl FromStr for OpMach {
    type Err = Error;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let err = format!("Error: {} is not valid machine operation", input);
        if let Ok(op) = OpLut::from_str(input) {
            Ok(OpMach::from(op))
        } else if let Ok(op) = OpReg::from_str(input) {
            Ok(OpMach::from(op))
        } else if let Ok(op) = OpDsp::from_str(input) {
            Ok(OpMach::from(op))
        } else if let Ok(op) = OpCarry::from_str(input) {
            Ok(OpMach::from(op))
        } else {
            Err(Error::new_parse_error(&err))
        }
    }
}
