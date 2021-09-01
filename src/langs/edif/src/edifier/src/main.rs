/*
Copyright 2021 Pedro M. Torruella N.

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
*/
use std::path::Path;

use std::fs::File;
use std::io::Write;

use edifier::ast::*;

static DOFILE: bool = false;

fn main() {
    let point = Edif {
        name: r#"dsp2"#.to_string(),
        elements: EdifElements::from(Vec::new()),
    };

    //point.libraries.push(mylib);

    let serialized = serde_sexpr::to_string(&point).unwrap();

    println!("serialized = {}", serialized);

    let path = Path::new("test.edf");
    let display = path.display();

    if DOFILE {
        // Open a file in write-only mode, returns `io::Result<File>`
        let mut file = match File::create(&path) {
            Err(why) => panic!("couldn't create {}: {}", display, why),
            Ok(file) => file,
        };

        match file.write_all(serialized.as_bytes()) {
            Err(why) => panic!("couldn't write to {}: {}", display, why),
            Ok(_) => println!("successfully wrote to {}", display),
        }
    }
}
