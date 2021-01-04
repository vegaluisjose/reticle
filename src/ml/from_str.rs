use crate::ml::ast::*;
use crate::util::errors::Error;
use std::str::FromStr;

impl FromStr for Opt {
    type Err = Error;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let err = format!("Error: {} is not valid lut operation", input);
        match input {
            "ra" => Ok(Opt::RegA),
            "rb" => Ok(Opt::RegB),
            "rc" => Ok(Opt::RegC),
            "rm" => Ok(Opt::RegM),
            "rp" => Ok(Opt::RegP),
            "op" => Ok(Opt::Op),
            "tbl" => Ok(Opt::Table),
            _ => Err(Error::new_parse_error(&err)),
        }
    }
}

impl FromStr for OpDsp {
    type Err = Error;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let err = format!("Error: {} is not valid dsp operation", input);
        match input {
            "add" => Ok(OpDsp::Add),
            "mul" => Ok(OpDsp::Mul),
            "muladd" => Ok(OpDsp::MulAdd),
            _ => Err(Error::new_parse_error(&err)),
        }
    }
}

impl FromStr for OpBasc {
    type Err = Error;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let err = format!("Error: {} is not valid basic operation", input);
        match input {
            "id" => Ok(OpBasc::Id),
            "gnd" => Ok(OpBasc::Gnd),
            "vcc" => Ok(OpBasc::Vcc),
            "ext" => Ok(OpBasc::Ext),
            "cat" => Ok(OpBasc::Cat),
            _ => Err(Error::new_parse_error(&err)),
        }
    }
}

impl FromStr for OpMach {
    type Err = Error;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let err = format!("Error: {} is not valid machine operation", input);
        match input {
            "lut1" => Ok(OpMach::Lut1),
            "lut2" => Ok(OpMach::Lut2),
            "lut3" => Ok(OpMach::Lut3),
            "lut4" => Ok(OpMach::Lut4),
            "lut5" => Ok(OpMach::Lut5),
            "lut6" => Ok(OpMach::Lut6),
            "fdre" => Ok(OpMach::Fdre),
            "fdse" => Ok(OpMach::Fdse),
            "dsp" => Ok(OpMach::Dsp),
            "carry" => Ok(OpMach::Carry),
            _ => Err(Error::new_parse_error(&err)),
        }
    }
}

impl FromStr for BelLut {
    type Err = Error;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let err = format!("Error: {} is not valid lut bel", input);
        match input {
            "a5lut" => Ok(BelLut::A5),
            "b5lut" => Ok(BelLut::B5),
            "c5lut" => Ok(BelLut::C5),
            "d5lut" => Ok(BelLut::D5),
            "e5lut" => Ok(BelLut::E5),
            "f5lut" => Ok(BelLut::F5),
            "g5lut" => Ok(BelLut::G5),
            "h5lut" => Ok(BelLut::H5),
            "a6lut" => Ok(BelLut::A6),
            "b6lut" => Ok(BelLut::B6),
            "c6lut" => Ok(BelLut::C6),
            "d6lut" => Ok(BelLut::D6),
            "e6lut" => Ok(BelLut::E6),
            "f6lut" => Ok(BelLut::F6),
            "g6lut" => Ok(BelLut::G6),
            "h6lut" => Ok(BelLut::H6),
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

impl FromStr for BelDsp {
    type Err = Error;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let err = format!("Error: {} is not valid dsp bel", input);
        match input {
            "alu" => Ok(BelDsp::Alu),
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
        } else if let Ok(bel) = BelDsp::from_str(input) {
            Ok(Bel::from(bel))
        } else {
            Err(Error::new_parse_error(&err))
        }
    }
}
