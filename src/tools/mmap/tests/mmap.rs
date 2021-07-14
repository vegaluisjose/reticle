use mmap::{Mem, Mmap};
use serde_json::{json, Result};

#[test]
fn test_one_empty_memory() -> Result<()> {
    let mut res = Mmap::new();
    res.insert("m0".into(), Mem::default());
    let mem = json!({"m0":{"offset": 0, "values": []}});
    let exp: Mmap = serde_json::from_value(mem)?;
    assert_eq!(res, exp);
    Ok(())
}

#[test]
fn test_one_memory() -> Result<()> {
    let mut res = Mmap::new();
    let mut m_0 = Mem::default();
    m_0.add_value(1);
    m_0.add_value(9);
    m_0.add_value(10);
    res.insert("m0".into(), m_0);
    let mem = json!({"m0":{"offset": 0, "values": [1, 9, 10]}});
    let exp: Mmap = serde_json::from_value(mem)?;
    assert_eq!(res, exp);
    Ok(())
}

#[test]
fn test_two_memory() -> Result<()> {
    let mut res = Mmap::new();
    let mut m_0 = Mem::default();
    let mut m_1 = Mem::default();
    m_0.add_value(1);
    m_0.add_value(9);
    m_0.add_value(10);
    m_1.add_value(3);
    res.insert("m0".into(), m_0);
    res.insert("m1".into(), m_1);
    let mem = json!({"m0":{"offset": 0, "values": [1, 9, 10]}, "m1":{"offset": 0, "values": [3]}});
    let exp: Mmap = serde_json::from_value(mem)?;
    assert_eq!(res, exp);
    Ok(())
}
