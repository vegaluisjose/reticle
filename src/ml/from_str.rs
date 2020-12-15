use crate::ml::ast::*;
use crate::util::errors::Error;
use std::str::FromStr;

impl FromStr for OptDsp {
    type Err = Error;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let err = format!("Error: {} is not valid lut operation", input);
        match input {
            "ra" => Ok(OptDsp::Ra),
            "rb" => Ok(OptDsp::Rb),
            "rc" => Ok(OptDsp::Rc),
            "rm" => Ok(OptDsp::Rm),
            "rp" => Ok(OptDsp::Rp),
            _ => Err(Error::new_parse_error(&err)),
        }
    }
}

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

impl FromStr for BelLut {
    type Err = Error;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let err = format!("Error: {} is not valid lut bel", input);
        match input {
            "a5" => Ok(BelLut::A5),
            "b5" => Ok(BelLut::B5),
            "c5" => Ok(BelLut::C5),
            "d5" => Ok(BelLut::D5),
            "e5" => Ok(BelLut::E5),
            "f5" => Ok(BelLut::F5),
            "g5" => Ok(BelLut::G5),
            "h5" => Ok(BelLut::H5),
            "a6" => Ok(BelLut::A6),
            "b6" => Ok(BelLut::B6),
            "c6" => Ok(BelLut::C6),
            "d6" => Ok(BelLut::D6),
            "e6" => Ok(BelLut::E6),
            "f6" => Ok(BelLut::F6),
            "g6" => Ok(BelLut::G6),
            "h6" => Ok(BelLut::H6),
            _ => Err(Error::new_parse_error(&err)),
        }
    }
}

impl FromStr for BelReg {
    type Err = Error;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let err = format!("Error: {} is not valid reg bel", input);
        match input {
            "aff" => Ok(BelReg::A),
            "bff" => Ok(BelReg::B),
            "cff" => Ok(BelReg::C),
            "dff" => Ok(BelReg::D),
            "eff" => Ok(BelReg::E),
            "fff" => Ok(BelReg::F),
            "gff" => Ok(BelReg::G),
            "hff" => Ok(BelReg::H),
            "aff2" => Ok(BelReg::A2),
            "bff2" => Ok(BelReg::B2),
            "cff2" => Ok(BelReg::C2),
            "dff2" => Ok(BelReg::D2),
            "eff2" => Ok(BelReg::E2),
            "fff2" => Ok(BelReg::F2),
            "gff2" => Ok(BelReg::G2),
            "hff2" => Ok(BelReg::H2),
            _ => Err(Error::new_parse_error(&err)),
        }
    }
}

impl FromStr for BelCarry {
    type Err = Error;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let err = format!("Error: {} is not valid carry bel", input);
        match input {
            "carry8" => Ok(BelCarry::Carry8),
            "carry4" => Ok(BelCarry::Carry4),
            _ => Err(Error::new_parse_error(&err)),
        }
    }
}

impl FromStr for Bel {
    type Err = Error;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let err = format!("Error: {} is not valid bel", input);
        if let Ok(bel) = BelLut::from_str(input) {
            Ok(Bel::from(bel))
        } else if let Ok(bel) = BelReg::from_str(input) {
            Ok(Bel::from(bel))
        } else if let Ok(bel) = BelCarry::from_str(input) {
            Ok(Bel::from(bel))
        } else {
            Err(Error::new_parse_error(&err))
        }
    }
}
