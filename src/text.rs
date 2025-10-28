use crate::emulator::instruction::Instruction;

pub fn reg(r: &str) -> u8 {
    match r.to_lowercase().as_str() {
        "rx" => 0,
        "ry" => 1,
        "rz" => 2,
        "ra" => 3,
        _ => 0,
    }
}

pub fn num(r: &str) -> usize {
    r.parse().unwrap()
}

pub fn from_text(s: &str) -> Vec<Instruction> {
    s.lines()
        .filter_map(|v| {
            let (name, args) = v.split_once(' ')?;
            let args: Vec<&str> = args.split(',').map(str::trim).collect();

            match name.to_lowercase().as_str() {
                "hlt" => Some(Instruction::HLT),
                "reg" => Some(Instruction::REG(reg(args[0]), num(args[1]))),
                "add" => Some(Instruction::ADD(reg(args[0]), reg(args[1]), reg(args[2]))),
                _ => None,
            }
        })
        .collect()
}
