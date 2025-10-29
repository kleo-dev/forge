mod binary;
mod emulator;

use std::{collections::HashMap, fs, sync::Arc};

use osui::prelude::*;

use crate::emulator::{instruction::Instruction, registry::Registry};

fn main() {
    let screen = Screen::new();
    let s = fs::read_to_string("input.fg").unwrap();

    app(&screen, Instruction::from_text(&s)).draw(&screen);

    screen.run().unwrap();
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
