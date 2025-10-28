mod binary;
mod emulator;
mod text;

use std::{fs, sync::Arc};

use osui::prelude::*;

use crate::{
    emulator::{instruction::Instruction, registry::Registry},
    text::from_text,
};

fn main() {
    let screen = Screen::new();
    let s = fs::read_to_string("input.fg").unwrap();

    app(&screen, from_text(&s)).draw(&screen);

    screen.run().unwrap();
}

fn app(_screen: &Arc<Screen>, instructions: Vec<Instruction>) -> Rsx {
    let pc = use_state::<usize>(0);
    let inst = use_state::<Instruction>(Instruction::HLT);
    let registers = Registry(use_state([0; 8]));

    let emulator = emulator::Emulator {
        pc: pc.clone(),
        registers: registers.clone(),
        inst: inst.clone(),
        instructions,
    };

    std::thread::spawn(move || emulator.run());

    rsx! {
        FlexRow, gap: 1, {
            @Style { foreground: None, background: Background::RoundedOutline(0x0000ff) };
            @Transform::new().padding(1, 1);
            %pc
            "Program Counter: {pc}"

            @Style { foreground: None, background: Background::RoundedOutline(0xff0000) };
            @Transform::new().padding(1, 1);
            %registers
            "{registers}"
        }

        @Style { foreground: None, background: Background::RoundedOutline(0x0000ff) };
        @Transform::center().padding(2, 2);
        %inst
        "Current instruction: {inst}"
    }
}
