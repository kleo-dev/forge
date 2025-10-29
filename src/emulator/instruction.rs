use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum Instruction {
    Hlt,
    Reg(u8, usize),
    Add(u8, u8, u8),
    Sub(u8, u8, u8),
    Mul(u8, u8, u8),
    Div(u8, u8, u8),
    Label(String),
    Jmp(String),
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Instruction::Hlt => format!("HLT"),
                Instruction::Reg(a, b) => format!("REG {a}, {b}"),
                Instruction::Add(a, b, c) => format!("ADD {a}, {b}, {c}"),
                Instruction::Sub(a, b, c) => format!("SUB {a}, {b}, {c}"),
                Instruction::Mul(a, b, c) => format!("MUL {a}, {b}, {c}"),
                Instruction::Div(a, b, c) => format!("DIV {a}, {b}, {c}"),
                Instruction::Label(a) => format!("LABEL {a}"),
                Instruction::Jmp(a) => format!("JMP {a}"),
            }
        )
    }
}

impl Instruction {
    pub fn from_text(s: &str) -> Vec<Instruction> {
        s.lines()
            .filter_map(|v| {
                let (name, args) = v.split_once(' ')?;
                let args: Vec<&str> = args.split(',').map(str::trim).collect();

                match name.to_lowercase().as_str() {
                    "hlt" => Some(Instruction::Hlt),
                    "reg" => Some(Instruction::Reg(reg(args[0]), num(args[1]))),
                    "add" => Some(Instruction::Add(reg(args[0]), reg(args[1]), reg(args[2]))),
                    "sub" => Some(Instruction::Sub(reg(args[0]), reg(args[1]), reg(args[2]))),
                    "mul" => Some(Instruction::Mul(reg(args[0]), reg(args[1]), reg(args[2]))),
                    "div" => Some(Instruction::Div(reg(args[0]), reg(args[1]), reg(args[2]))),
                    "label" => Some(Instruction::Label(args[0].to_string())),
                    "jmp" => Some(Instruction::Jmp(args[0].to_string())),
                    _ => None,
                }
            })
            .collect()
    }
}

fn reg(r: &str) -> u8 {
    match r.to_lowercase().as_str() {
        "ra" => 0,
        "rb" => 1,
        "rc" => 2,
        "rd" => 3,
        _ => 0,
    }
}

fn num(r: &str) -> usize {
    r.parse().unwrap()
}
