use std::cmp::Ordering;

pub fn match_check(incoming: String) -> i32 {
    let mut counter: i32 = 0;
    for c in incoming.chars() {
        if c == '(' {
            counter += 1;
        } else if c == ')' {
            counter -= 1;
        }
    }
    match counter.cmp(&0) {
        Ordering::Greater => println!("ERROR: Too many left parentheses."),
        Ordering::Less => println!("ERROR: Too many right parentheses."),
        Ordering::Equal => (),
    }
    counter
}

pub fn add_new_lines(input: String, level: u32, dospaces: bool) -> String {
    let mut output = String::new();
    let mut count = 0;
    let mut first_pass = false;
    for c in input.chars() {
        match c {
            '(' => {
                if (count <= level) & (first_pass) {
                    if output.ends_with(' ') {
                        output.pop();
                    }
                    output.push('\n');
                    if dospaces {
                        for _i in 0..count {
                            output.push(' ');
                            output.push(' ');
                        }
                    }
                }
                output.push(c);
                first_pass = true;
                count += 1;
            }
            ')' => {
                output.push(c);
                count -= 1;
            }
            _ => output.push(c),
        }
    }
    assert_eq!(0, match_check(output.clone()));
    output
}
