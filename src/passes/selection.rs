use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub enum Opcode {
    Ref,
    Add,
    Mul,
    Reg,
}

impl fmt::Display for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Opcode::Ref => write!(f, "ref"),
            Opcode::Add => write!(f, "add"),
            Opcode::Mul => write!(f, "mul"),
            Opcode::Reg => write!(f, "reg"),
        }
    }
}

#[derive(Clone, Debug)]
pub enum Loc {
    Placeholder,
    Dsp,
    Lut,
}

impl fmt::Display for Loc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Loc::Placeholder => write!(f, "??"),
            Loc::Dsp => write!(f, "dsp"),
            Loc::Lut => write!(f, "lut"),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Node {
    opcode: Opcode,
    operands: Vec<Node>,
    width: u64,
    loc: Loc,
    cost: u128,
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        if self.opcode != other.opcode {
            false
        } else if self.width != other.width {
            false
        } else if self.operands.len() != other.operands.len() {
            false
        } else {
            let mut eq = true;
            for (a, b) in self.operands.iter().zip(other.operands.iter()) {
                if a.opcode != b.opcode {
                    eq = false;
                    break;
                } else if a.width != b.width {
                    eq = false;
                    break;
                } else if a.operands.len() != b.operands.len() {
                    eq = false;
                    break;
                }
            }
            eq
        }
    }
}

impl Eq for Node {}

impl Node {
    pub fn new_with_attrs(opcode: &Opcode, width: u64, loc: &Loc, cost: u128) -> Node {
        Node {
            opcode: opcode.clone(),
            operands: Vec::new(),
            width: width.clone(),
            loc: loc.clone(),
            cost: cost,
        }
    }

    pub fn change_cost(&mut self, cost: u128) -> &mut Node {
        self.cost = cost;
        self
    }

    pub fn push_operand(&mut self, operand: &Node) -> &mut Node {
        self.operands.push(operand.clone());
        self
    }

    pub fn has_placeholder(&self) -> bool {
        match self.loc {
            Loc::Placeholder => true,
            _ => false,
        }
    }

    pub fn postorder(&self) -> Vec<Node> {
        let mut stack: Vec<Node> = Vec::new();
        let mut res: Vec<Node> = Vec::new();
        stack.push(self.clone());
        while !stack.is_empty() {
            let node = stack.pop().unwrap();
            res.push(node.clone());
            for operand in node.operands.iter() {
                stack.push(operand.clone());
            }
        }
        res.reverse();
        res
    }

    pub fn estimate(&self) -> u128 {
        let mut sum = self.cost;
        for operand in self.operands.iter() {
            sum += operand.estimate();
        }
        sum
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "opcode:{} loc:{} cost:{}",
            self.opcode, self.loc, self.cost
        )
    }
}

fn instruction_selection(code: &Node, patterns: &Vec<Node>) -> Vec<Node> {
    let mut codegen: Vec<Node> = Vec::new();
    for instr in code.postorder().iter() {
        let mut best = u128::max_value();
        let mut found = false;
        for pat in patterns.iter() {
            if instr == pat {
                if best == u128::max_value() {
                    best = instr.estimate();
                }
                let cost = pat.estimate();
                if cost < best && instr.has_placeholder() {
                    if found {
                        codegen.pop().unwrap();
                    }
                    codegen.push(pat.clone());
                    found = true;
                    best = cost;
                }
            }
        }
        if !found {
            codegen.push(instr.clone());
        }
    }
    codegen
}

pub fn example() {
    let input = Node::new_with_attrs(&Opcode::Ref, 8, &Loc::Placeholder, 0);
    let mut code = Node::new_with_attrs(&Opcode::Add, 8, &Loc::Placeholder, u128::max_value());
    let mut dsp_add = Node::new_with_attrs(&Opcode::Add, 8, &Loc::Dsp, 1);
    dsp_add.push_operand(&input);
    dsp_add.push_operand(&input);
    code.push_operand(&input);
    code.push_operand(&input);
    let mut patterns: Vec<Node> = Vec::new();
    patterns.push(input.clone());
    patterns.push(dsp_add.clone());
    let codegen = instruction_selection(&code, &patterns);
    println!("Before opt");
    for i in code.postorder().iter() {
        println!("{}", i);
    }
    println!("After opt");
    for i in codegen.iter() {
        println!("{}", i);
    }
}
