use bincode::{deserialize_from, serialize_into};
use reticle::tdl::ast as tdl;
use reticle::tdl::parser::TDLParser;
use reticle::util::errors::Error;
use reticle::util::file::create_abs_path;
use std::fs::File;
use std::io::{BufReader, BufWriter};

fn main() -> Result<(), Error> {
    let file = create_abs_path("examples/target/ultrascale/lut.tdl");
    let tdl = TDLParser::parse_from_file(file)?;
    let bin = create_abs_path("lut.bin");
    let mut f = BufWriter::new(File::create(&bin).unwrap());
    serialize_into(&mut f, &tdl).unwrap();
    drop(f);
    let mut br = BufReader::new(File::open(&bin).unwrap());
    let res: tdl::Target = deserialize_from(&mut br).unwrap();
    println!("{}", res);
    Ok(())
}
