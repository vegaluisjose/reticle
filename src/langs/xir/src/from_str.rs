use crate::ast::*;
use crate::errors::Error;
use std::str::FromStr;

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
            _ => Err(Error::new_conv_error(&err)),
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
            "carryadd" => Ok(OpMach::CarryAdd),
            "carrysub" => Ok(OpMach::CarrySub),
            "vaddrega" => Ok(OpMach::VecAddRegA),
            "vadd" => Ok(OpMach::VecAdd),
            "vsub" => Ok(OpMach::VecSub),
            "vmul" => Ok(OpMach::VecMul),
            "mul" => Ok(OpMach::Mul),
            "muladd" => Ok(OpMach::MulAdd),
            "muladdrega" => Ok(OpMach::MulAddRegA),
            "muladdregaci" => Ok(OpMach::MulAddRegACi),
            "muladdregaco" => Ok(OpMach::MulAddRegACo),
            "muladdregacio" => Ok(OpMach::MulAddRegACio),
            "lram" => Ok(OpMach::Lram),
            "bram" => Ok(OpMach::Bram),
            "lrom" => Ok(OpMach::Lrom),
            "brom" => Ok(OpMach::Brom),
            _ => Err(Error::new_conv_error(&err)),
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
            _ => Err(Error::new_conv_error(&err)),
        }
    }
}

impl FromStr for BelReg {
    type Err = Error;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let err = format!("Error: {} is not valid reg bel", input);
        match input {
            "a" => Ok(BelReg::A),
            "b" => Ok(BelReg::B),
            "c" => Ok(BelReg::C),
            "d" => Ok(BelReg::D),
            "e" => Ok(BelReg::E),
            "f" => Ok(BelReg::F),
            "g" => Ok(BelReg::G),
            "h" => Ok(BelReg::H),
            "a2" => Ok(BelReg::A2),
            "b2" => Ok(BelReg::B2),
            "c2" => Ok(BelReg::C2),
            "d2" => Ok(BelReg::D2),
            "e2" => Ok(BelReg::E2),
            "f2" => Ok(BelReg::F2),
            "g2" => Ok(BelReg::G2),
            "h2" => Ok(BelReg::H2),
            _ => Err(Error::new_conv_error(&err)),
        }
    }
}

impl FromStr for BelCarry {
    type Err = Error;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let err = format!("Error: {} is not valid carry bel", input);
        match input {
            "c8" => Ok(BelCarry::Carry8),
            "c4" => Ok(BelCarry::Carry4),
            _ => Err(Error::new_conv_error(&err)),
        }
    }
}

impl FromStr for BelDsp {
    type Err = Error;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let err = format!("Error: {} is not valid dsp bel", input);
        match input {
            "alu" => Ok(BelDsp::Alu),
            _ => Err(Error::new_conv_error(&err)),
        }
    }
}

impl FromStr for BelBlock {
    type Err = Error;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let err = format!("Error: {} is not valid block bel", input);
        match input {
            "u" => Ok(BelBlock::U),
            "l" => Ok(BelBlock::L),
            _ => Err(Error::new_conv_error(&err)),
        }
    }
}

impl FromStr for BelLum {
    type Err = Error;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let err = format!("Error: {} is not valid lum bel", input);
        match input {
            "h6" => Ok(BelLum::H6),
            _ => Err(Error::new_conv_error(&err)),
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
        } else if let Ok(bel) = BelLum::from_str(input) {
            Ok(Bel::from(bel))
        } else {
            Err(Error::new_conv_error(&err))
        }
    }
}
