use crate::verilog::ast as vl;
use crate::xml::ast as xml;

impl From<xml::OpMach> for vl::Id {
    fn from(op: xml::OpMach) -> Self {
        match op {
            xml::OpMach::Lut1 => "LUT1".to_string(),
            xml::OpMach::Lut2 => "LUT2".to_string(),
            xml::OpMach::Lut3 => "LUT3".to_string(),
            xml::OpMach::Lut4 => "LUT4".to_string(),
            xml::OpMach::Lut5 => "LUT5".to_string(),
            xml::OpMach::Lut6 => "LUT6".to_string(),
            xml::OpMach::Fdre => "FDRE".to_string(),
            xml::OpMach::Fdse => "FDSE".to_string(),
            xml::OpMach::Dsp => "DSP48E2".to_string(),
            xml::OpMach::Carry => "CARRY8".to_string(),
        }
    }
}

impl From<xml::BelLut> for vl::Id {
    fn from(bel: xml::BelLut) -> Self {
        match bel {
            xml::BelLut::A5 => "A5LUT".to_string(),
            xml::BelLut::B5 => "B5LUT".to_string(),
            xml::BelLut::C5 => "C5LUT".to_string(),
            xml::BelLut::D5 => "D5LUT".to_string(),
            xml::BelLut::E5 => "E5LUT".to_string(),
            xml::BelLut::F5 => "F5LUT".to_string(),
            xml::BelLut::G5 => "G5LUT".to_string(),
            xml::BelLut::H5 => "H5LUT".to_string(),
            xml::BelLut::A6 => "A6LUT".to_string(),
            xml::BelLut::B6 => "B6LUT".to_string(),
            xml::BelLut::C6 => "C6LUT".to_string(),
            xml::BelLut::D6 => "D6LUT".to_string(),
            xml::BelLut::E6 => "E6LUT".to_string(),
            xml::BelLut::F6 => "F6LUT".to_string(),
            xml::BelLut::G6 => "G6LUT".to_string(),
            xml::BelLut::H6 => "H6LUT".to_string(),
        }
    }
}

impl From<xml::BelReg> for vl::Id {
    fn from(bel: xml::BelReg) -> Self {
        match bel {
            xml::BelReg::A => "AFF".to_string(),
            xml::BelReg::B => "BFF".to_string(),
            xml::BelReg::C => "CFF".to_string(),
            xml::BelReg::D => "DFF".to_string(),
            xml::BelReg::E => "EFF".to_string(),
            xml::BelReg::F => "FFF".to_string(),
            xml::BelReg::G => "GFF".to_string(),
            xml::BelReg::H => "HFF".to_string(),
            xml::BelReg::A2 => "AFF2".to_string(),
            xml::BelReg::B2 => "BFF2".to_string(),
            xml::BelReg::C2 => "CFF2".to_string(),
            xml::BelReg::D2 => "DFF2".to_string(),
            xml::BelReg::E2 => "EFF2".to_string(),
            xml::BelReg::F2 => "FFF2".to_string(),
            xml::BelReg::G2 => "GFF2".to_string(),
            xml::BelReg::H2 => "HFF2".to_string(),
        }
    }
}

impl From<xml::BelCarry> for vl::Id {
    fn from(bel: xml::BelCarry) -> Self {
        match bel {
            xml::BelCarry::Carry4 => "CARRY4".to_string(),
            xml::BelCarry::Carry8 => "CARRY8".to_string(),
        }
    }
}

impl From<xml::BelDsp> for vl::Id {
    fn from(bel: xml::BelDsp) -> Self {
        match bel {
            xml::BelDsp::Alu => "DSP_ALU".to_string(),
        }
    }
}

impl From<xml::Bel> for vl::Id {
    fn from(bel: xml::Bel) -> Self {
        match bel {
            xml::Bel::Lut(lut) => lut.into(),
            xml::Bel::Reg(reg) => reg.into(),
            xml::Bel::Carry(carry) => carry.into(),
            xml::Bel::Dsp(dsp) => dsp.into(),
        }
    }
}

impl From<xml::Bel> for vl::AttributeTy {
    fn from(bel: xml::Bel) -> Self {
        let val = vl::Id::from(bel);
        vl::AttributeTy::new_stmt("BEL", &val)
    }
}

impl From<xml::Loc> for vl::AttributeTy {
    fn from(loc: xml::Loc) -> Self {
        let val = match loc.bel() {
            xml::Bel::Dsp(_) => format!("DSP48E2_X{}Y{}", loc.x(), loc.y()),
            _ => format!("SLICE_X{}Y{}", loc.x(), loc.y()),
        };
        vl::AttributeTy::new_stmt("LOC", &val)
    }
}

impl From<xml::Loc> for vl::Attribute {
    fn from(loc: xml::Loc) -> Self {
        let mut attr = vl::Attribute::default();
        let site = vl::AttributeTy::from(loc.bel().clone());
        let bel = vl::AttributeTy::from(loc);
        attr.add_attr(site);
        attr.add_attr(bel);
        attr
    }
}
