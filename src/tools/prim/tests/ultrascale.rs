use anyhow::Result;
use prim::ultrascale::carry::{Carry, CarryParam, CarryType};
use prim::{Param, ParamSet, PortSet, Prim};
use std::fmt;
use std::iter::FromIterator;

fn test_name<T: Eq + Default + fmt::Debug + fmt::Display>(prim: &Prim<T>, exp: &str) {
    let res = prim.name();
    assert_eq!(res, exp);
}

fn test_param<T: Eq + Default + fmt::Debug + fmt::Display>(prim: &Prim<T>, exp: &ParamSet<T>) {
    let res = prim.param();
    let res = Vec::from_iter(res.iter());
    let exp = Vec::from_iter(exp.iter());
    assert_eq!(res, exp);
}

fn test_input<T: Eq + Default + fmt::Debug + fmt::Display>(prim: &Prim<T>, exp: &[(&str, u32)]) {
    let res = prim.input();
    let exp = PortSet::from(exp);
    assert_eq!(*res, exp);
}

fn test_output<T: Eq + Default + fmt::Debug + fmt::Display>(prim: &Prim<T>, exp: &[(&str, u32)]) {
    let res = prim.output();
    let exp = PortSet::from(exp);
    assert_eq!(*res, exp);
}

#[test]
fn test_carry_name() {
    let carry = Carry::default();
    test_name(&carry, "CARRY8");
}

#[test]
fn test_carry_param() {
    let carry = Carry::default();
    let mut param: ParamSet<CarryParam> = ParamSet::new();
    param.insert(Param {
        name: "CARRY_TYPE".to_string(),
        width: None,
        value: CarryType::Single.into(),
    });
    test_param(&carry, &param);
}

#[test]
fn test_carry_input() {
    let carry = Carry::default();
    let input = [("DI", 8), ("S", 8), ("CI", 1), ("CI_TOP", 1)];
    test_input(&carry, &input);
}

#[test]
fn test_carry_output() {
    let carry = Carry::default();
    let output = [("O", 8), ("CO", 8)];
    test_output(&carry, &output);
}

#[test]
fn test_carry_set_param() -> Result<()> {
    let mut carry = Carry::default();
    carry.set_param("CARRY_TYPE", CarryType::Dual)?;
    Ok(())
}
