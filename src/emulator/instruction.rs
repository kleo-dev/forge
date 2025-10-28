use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum Instruction {
    HLT,
    REG(u8, usize),
    ADD(u8, u8, u8),
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Instruction::HLT => format!("HLT"),
                Instruction::REG(a, b) => format!("REG {a}, {b}"),
                Instruction::ADD(a, b, c) => format!("ADD {a}, {b}, {c}"),
            }
        )
    }
}
