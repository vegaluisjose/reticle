use crate::asm::ast::*;
use crate::util::errors::Error;
use crate::util::file::{create_abs_path, remove_tmp_file, write_to_tmp_file};
use std::collections::HashMap;
use std::fmt;
use std::process::Command;

#[derive(Clone, Debug)]
pub struct Constraint {
    pub prim: Prim,
    pub count: u64,
    pub left: HashMap<String, u64>,
    pub right: HashMap<u64, String>,
}

impl Default for Constraint {
    fn default() -> Self {
        Constraint {
            prim: Prim::Any,
            count: 0,
            left: HashMap::new(),
            right: HashMap::new(),
        }
    }
}

impl Constraint {
    pub fn new_lut() -> Constraint {
        let mut con = Constraint::default();
        con.set_prim(Prim::Lut);
        con
    }
    pub fn new_dsp() -> Constraint {
        let mut con = Constraint::default();
        con.set_prim(Prim::Dsp);
        con
    }
    pub fn prim(&self) -> &Prim {
        &self.prim
    }
    pub fn get(&self, index: &u64) -> Option<&String> {
        self.right.get(index)
    }
    pub fn set_prim(&mut self, prim: Prim) {
        self.prim = prim;
    }
    pub fn add(&mut self, name: &str) {
        self.left.insert(name.to_string(), self.count);
        self.right.insert(self.count, name.to_string());
        self.count += 1;
    }
}

impl fmt::Display for Constraint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut contents = String::new();
        for index in self.left.values() {
            let line = format!("{},{}\n", index, self.prim);
            contents.push_str(&line);
        }
        write!(f, "{}", contents)
    }
}

fn parse_from_line(line: &[&str]) -> Result<(u64, u64, u64), Error> {
    if line.len() == 3 {
        let i = line[0].parse::<u64>()?;
        let x = line[1].parse::<u64>()?;
        let y = line[2].parse::<u64>()?;
        Ok((i, x, y))
    } else {
        Err(Error::new_placer_error("wrong result format"))
    }
}

fn place(con: &Constraint) -> Result<HashMap<String, Loc>, Error> {
    let mut res: HashMap<String, Loc> = HashMap::new();
    let bin = create_abs_path("src/placer/z3_placer.py");
    let file = "__reticle_constraints.txt";
    let path = write_to_tmp_file(&file, &con.to_string());
    let cmd = Command::new("python3")
        .arg(bin)
        .arg(&path)
        .arg("-p")
        .arg(con.prim().to_string())
        .output()
        .expect("failed to execute placement");
    let out = String::from_utf8_lossy(&cmd.stdout).to_string();
    // println!("input:\n{}", con);
    // println!("output:\n{}", out)
    for line in out.lines() {
        let values: Vec<&str> = line.split(',').collect();
        let (i, x, y) = parse_from_line(&values)?;
        if let Some(id) = con.get(&i) {
            let loc = Loc {
                prim: con.prim().clone(),
                x: ExprCoord::Val(x),
                y: ExprCoord::Val(y),
            };
            res.insert(id.to_string(), loc);
        }
    }
    remove_tmp_file(path);
    Ok(res)
}

pub fn place_from_prog(prog: &Prog) -> Result<(), Error> {
    let mut dsp = Constraint::new_dsp();
    let mut lut = Constraint::new_lut();
    for instr in prog.body() {
        if let Instr::Asm(asm) = instr {
            let id = asm.dst().get_id(0)?;
            if asm.is_dsp() {
                dsp.add(&id);
            } else {
                lut.add(&id);
            }
        }
    }
    let res = place(&lut)?;
    for (id, loc) in res {
        println!("{} --> {}", id, loc);
    }
    Ok(())
}
