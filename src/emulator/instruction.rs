use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum Instruction {
    Hlt,
    Reg(u8, usize),
    Add(u8, u8, u8),
    Sub(u8, u8, u8),
    Mul(u8, u8, u8),
    Div(u8, u8, u8),
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
            }
        )
    }
}
