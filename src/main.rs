mod emulator;

use std::{
    collections::HashMap,
    env,
    fs::{self, File},
    io::Write,
    sync::Arc,
};

use osui::prelude::*;

use crate::emulator::{instruction::Instruction, registry::Registry};

fn main() {
    let args: Vec<_> = env::args().collect();
    if let Some(file) = args.get(1) {
        if file.ends_with(".fg") {
            let s = fs::read_to_string(file).unwrap();
            let instructions = Instruction::from_text(&s);
            let mut bytes = Vec::new();
            for (i, instr) in instructions.iter().enumerate() {
                bytes.extend(instr.encode());
                if i != instructions.len() - 1 {
                    bytes.push(0x0D);
                }
            }
            let mut file = File::create("out.bin").unwrap();
            file.write_all(&bytes).unwrap();

            println!("Compiled successfully");
        } else {
            let bytes = fs::read(file).unwrap();

            let mut instructions = Vec::new();
            for chunk in bytes.split(|b| *b == 0x0D) {
                if let Some(instr) = Instruction::decode(chunk) {
                    instructions.push(instr);
                }
            }

            let screen = Screen::new();

            app(&screen, instructions).draw(&screen);

            screen.run().unwrap();
        }
    } else {
        eprintln!("\x1b[31mERR: Please enter a file\x1b[0m")
    }
}

fn app(_screen: &Arc<Screen>, instructions: Vec<Instruction>) -> Rsx {
    let pc = use_state::<usize>(0);
    let inst = use_state::<Instruction>(Instruction::Hlt);
    let registers = Registry(use_state([0; 8]));
    let labels = use_state(HashMap::new());

    let emulator = emulator::Emulator {
        pc: pc.clone(),
        registers: registers.clone(),
        inst: inst.clone(),
        labels: labels.clone(),
        instructions,
    };

    std::thread::spawn(move || emulator.run());

    rsx! {
        FlexRow, gap: 1, {
            @Style { foreground: None, background: Background::RoundedOutline(0x0000ff) };
            @Transform::new().padding(1, 1);
            FlexCol, gap: 1, {
                %pc
                "Program Counter: {pc}"

                "|"

                %labels
                "Labels: {labels:?}"
            }

            @Style { foreground: None, background: Background::RoundedOutline(0xff0000) };
            @Transform::new().padding(1, 1);
            FlexCol, gap: 1, {
                %registers
                "{registers}"

                "|"

                %inst
                "Current instruction: {inst}"
            }
        }
    }
}
