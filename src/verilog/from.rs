use crate::verilog::ast as vl;
use crate::xl::ast as xl;

impl From<xl::OpMach> for vl::Id {
    fn from(op: xl::OpMach) -> Self {
        match op {
            xl::OpMach::Lut1 => "LUT1".to_string(),
            xl::OpMach::Lut2 => "LUT2".to_string(),
            xl::OpMach::Lut3 => "LUT3".to_string(),
            xl::OpMach::Lut4 => "LUT4".to_string(),
            xl::OpMach::Lut5 => "LUT5".to_string(),
            xl::OpMach::Lut6 => "LUT6".to_string(),
            xl::OpMach::Fdre => "FDRE".to_string(),
            xl::OpMach::Fdse => "FDSE".to_string(),
            xl::OpMach::Dsp => "DSP48E2".to_string(),
            xl::OpMach::Carry => "CARRY8".to_string(),
        }
    }
}

impl From<xl::BelLut> for vl::Id {
    fn from(bel: xl::BelLut) -> Self {
        match bel {
            xl::BelLut::A5 => "A5LUT".to_string(),
            xl::BelLut::B5 => "B5LUT".to_string(),
            xl::BelLut::C5 => "C5LUT".to_string(),
            xl::BelLut::D5 => "D5LUT".to_string(),
            xl::BelLut::E5 => "E5LUT".to_string(),
            xl::BelLut::F5 => "F5LUT".to_string(),
            xl::BelLut::G5 => "G5LUT".to_string(),
            xl::BelLut::H5 => "H5LUT".to_string(),
            xl::BelLut::A6 => "A6LUT".to_string(),
            xl::BelLut::B6 => "B6LUT".to_string(),
            xl::BelLut::C6 => "C6LUT".to_string(),
            xl::BelLut::D6 => "D6LUT".to_string(),
            xl::BelLut::E6 => "E6LUT".to_string(),
            xl::BelLut::F6 => "F6LUT".to_string(),
            xl::BelLut::G6 => "G6LUT".to_string(),
            xl::BelLut::H6 => "H6LUT".to_string(),
        }
    }
}

impl From<xl::BelReg> for vl::Id {
    fn from(bel: xl::BelReg) -> Self {
        match bel {
            xl::BelReg::A => "AFF".to_string(),
            xl::BelReg::B => "BFF".to_string(),
            xl::BelReg::C => "CFF".to_string(),
            xl::BelReg::D => "DFF".to_string(),
            xl::BelReg::E => "EFF".to_string(),
            xl::BelReg::F => "FFF".to_string(),
            xl::BelReg::G => "GFF".to_string(),
            xl::BelReg::H => "HFF".to_string(),
            xl::BelReg::A2 => "AFF2".to_string(),
            xl::BelReg::B2 => "BFF2".to_string(),
            xl::BelReg::C2 => "CFF2".to_string(),
            xl::BelReg::D2 => "DFF2".to_string(),
            xl::BelReg::E2 => "EFF2".to_string(),
            xl::BelReg::F2 => "FFF2".to_string(),
            xl::BelReg::G2 => "GFF2".to_string(),
            xl::BelReg::H2 => "HFF2".to_string(),
        }
    }
}

impl From<xl::BelCarry> for vl::Id {
    fn from(bel: xl::BelCarry) -> Self {
        match bel {
            xl::BelCarry::Carry4 => "CARRY4".to_string(),
            xl::BelCarry::Carry8 => "CARRY8".to_string(),
        }
    }
}

impl From<xl::BelDsp> for vl::Id {
    fn from(bel: xl::BelDsp) -> Self {
        match bel {
            xl::BelDsp::Alu => "DSP_ALU".to_string(),
        }
    }
}

impl From<xl::Bel> for vl::Id {
    fn from(bel: xl::Bel) -> Self {
        match bel {
            xl::Bel::Lut(lut) => lut.into(),
            xl::Bel::Reg(reg) => reg.into(),
            xl::Bel::Carry(carry) => carry.into(),
            xl::Bel::Dsp(dsp) => dsp.into(),
        }
    }
}

impl From<xl::Bel> for vl::AttributeTy {
    fn from(bel: xl::Bel) -> Self {
        let val = vl::Id::from(bel);
        vl::AttributeTy::new_stmt("BEL", &val)
    }
}

impl From<xl::Loc> for vl::AttributeTy {
    fn from(loc: xl::Loc) -> Self {
        let val = match loc.bel() {
            xl::Bel::Dsp(_) => format!("DSP48E2_X{}Y{}", loc.x(), loc.y()),
            _ => format!("SLICE_X{}Y{}", loc.x(), loc.y()),
        };
        vl::AttributeTy::new_stmt("LOC", &val)
    }
}

impl From<xl::Loc> for vl::Attribute {
    fn from(loc: xl::Loc) -> Self {
        let mut attr = vl::Attribute::default();
        let site = vl::AttributeTy::from(loc.bel().clone());
        let bel = vl::AttributeTy::from(loc);
        attr.add_attr(site);
        attr.add_attr(bel);
        attr
    }
}
