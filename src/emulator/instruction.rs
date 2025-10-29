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

    pub fn encode(&self) -> Vec<u8> {
        match self {
            Instruction::Hlt => vec![0x1],

            // Register instruction
            Instruction::Reg(r, a) => {
                let mut bytes = vec![0x02, *r];
                bytes.extend_from_slice(&(*a as u64).to_le_bytes());
                bytes
            }

            // Arithmetic: opcode + 3 registers
            Instruction::Add(a, b, c) => vec![0x03, *a, *b, *c],
            Instruction::Sub(a, b, c) => vec![0x04, *a, *b, *c],
            Instruction::Mul(a, b, c) => vec![0x05, *a, *b, *c],
            Instruction::Div(a, b, c) => vec![0x06, *a, *b, *c],

            // Label: opcode + length + UTF-8 bytes
            Instruction::Label(name) => {
                let mut bytes = vec![0x07];
                let name_bytes = name.as_bytes();
                let len = name_bytes.len() as u16;
                bytes.extend_from_slice(&len.to_le_bytes());
                bytes.extend_from_slice(name_bytes);
                bytes
            }

            // Jump: opcode + length + UTF-8 bytes
            Instruction::Jmp(target) => {
                let mut bytes = vec![0x08];
                let name_bytes = target.as_bytes();
                let len = name_bytes.len() as u16;
                bytes.extend_from_slice(&len.to_le_bytes());
                bytes.extend_from_slice(name_bytes);
                bytes
            }
        }
    }

    pub fn decode(bytes: &[u8]) -> Option<Self> {
        let opcode = *bytes.get(0)?;
        match opcode {
            // Hlt
            0x01 => Some(Instruction::Hlt),

            // Reg(u8, usize)
            0x02 => {
                let r = *bytes.get(1)?;
                if bytes.len() < 10 {
                    return None;
                }
                let mut a_bytes = [0u8; 8];
                a_bytes.copy_from_slice(&bytes[2..10]);
                let a = u64::from_le_bytes(a_bytes) as usize;
                Some(Instruction::Reg(r, a))
            }

            // 3-register arithmetic
            0x03 => Some(Instruction::Add(
                *bytes.get(1)?,
                *bytes.get(2)?,
                *bytes.get(3)?,
            )),
            0x04 => Some(Instruction::Sub(
                *bytes.get(1)?,
                *bytes.get(2)?,
                *bytes.get(3)?,
            )),
            0x05 => Some(Instruction::Mul(
                *bytes.get(1)?,
                *bytes.get(2)?,
                *bytes.get(3)?,
            )),
            0x06 => Some(Instruction::Div(
                *bytes.get(1)?,
                *bytes.get(2)?,
                *bytes.get(3)?,
            )),

            // Label(String)
            0x07 => {
                if bytes.len() < 3 {
                    return None;
                }
                let len = u16::from_le_bytes([bytes[1], bytes[2]]) as usize;
                let text_bytes = &bytes[3..3 + len];
                let text = String::from_utf8(text_bytes.to_vec()).ok()?;
                Some(Instruction::Label(text))
            }

            // Jmp(String)
            0x08 => {
                if bytes.len() < 3 {
                    return None;
                }
                let len = u16::from_le_bytes([bytes[1], bytes[2]]) as usize;
                let text_bytes = &bytes[3..3 + len];
                let text = String::from_utf8(text_bytes.to_vec()).ok()?;
                Some(Instruction::Jmp(text))
            }

            _ => None,
        }
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
