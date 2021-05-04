use crate::{Bel, BelCarry, BelDsp, BelLut, BelReg, Loc};
use verilog::ast as vl;

fn string_from_bel_reg(bel: &BelReg) -> String {
    match bel {
        BelReg::A => "AFF".to_string(),
        BelReg::B => "BFF".to_string(),
        BelReg::C => "CFF".to_string(),
        BelReg::D => "DFF".to_string(),
        BelReg::E => "EFF".to_string(),
        BelReg::F => "FFF".to_string(),
        BelReg::G => "GFF".to_string(),
        BelReg::H => "HFF".to_string(),
        BelReg::A2 => "AFF2".to_string(),
        BelReg::B2 => "BFF2".to_string(),
        BelReg::C2 => "CFF2".to_string(),
        BelReg::D2 => "DFF2".to_string(),
        BelReg::E2 => "EFF2".to_string(),
        BelReg::F2 => "FFF2".to_string(),
        BelReg::G2 => "GFF2".to_string(),
        BelReg::H2 => "HFF2".to_string(),
    }
}

fn string_from_bel_lut(bel: &BelLut) -> String {
    match bel {
        BelLut::A5 => "A5LUT".to_string(),
        BelLut::B5 => "B5LUT".to_string(),
        BelLut::C5 => "C5LUT".to_string(),
        BelLut::D5 => "D5LUT".to_string(),
        BelLut::E5 => "E5LUT".to_string(),
        BelLut::F5 => "F5LUT".to_string(),
        BelLut::G5 => "G5LUT".to_string(),
        BelLut::H5 => "H5LUT".to_string(),
        BelLut::A6 => "A6LUT".to_string(),
        BelLut::B6 => "B6LUT".to_string(),
        BelLut::C6 => "C6LUT".to_string(),
        BelLut::D6 => "D6LUT".to_string(),
        BelLut::E6 => "E6LUT".to_string(),
        BelLut::F6 => "F6LUT".to_string(),
        BelLut::G6 => "G6LUT".to_string(),
        BelLut::H6 => "H6LUT".to_string(),
    }
}

fn string_from_bel_carry(bel: &BelCarry) -> String {
    match bel {
        BelCarry::Carry4 => "CARRY4".to_string(),
        BelCarry::Carry8 => "CARRY8".to_string(),
    }
}

fn string_from_bel_dsp(bel: &BelDsp) -> String {
    match bel {
        BelDsp::Alu => "DSP_ALU".to_string(),
    }
}

fn string_from_bel(bel: &Bel) -> String {
    match bel {
        Bel::Lut(lut) => string_from_bel_lut(lut),
        Bel::Reg(reg) => string_from_bel_reg(reg),
        Bel::Carry(carry) => string_from_bel_carry(carry),
        Bel::Dsp(dsp) => string_from_bel_dsp(dsp),
    }
}

fn attr_stmt_from_bel(bel: &Bel) -> vl::AttributeTy {
    let val = string_from_bel(bel);
    vl::AttributeTy::new_stmt("BEL", &val)
}

fn attr_stmt_from_loc(loc: &Loc) -> vl::AttributeTy {
    let val = match loc.bel() {
        Bel::Dsp(_) => format!("DSP48E2_X{}Y{}", loc.x(), loc.y()),
        _ => format!("SLICE_X{}Y{}", loc.x(), loc.y()),
    };
    vl::AttributeTy::new_stmt("LOC", &val)
}

pub fn attr_from_loc(loc: &Loc) -> vl::Attribute {
    let mut attr = vl::Attribute::default();
    let site = attr_stmt_from_bel(loc.bel());
    let bel = attr_stmt_from_loc(loc);
    attr.add_attr(site);
    attr.add_attr(bel);
    attr
}
