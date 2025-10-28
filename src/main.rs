mod binary;
mod emulator;
mod text;

use std::sync::Arc;

use osui::prelude::*;

fn main() {
    let screen = Screen::new();

    app(&screen).draw(&screen);

    screen.run().unwrap();
}

fn app(_screen: &Arc<Screen>) -> Rsx {
    let pc = use_state::<u32>(0);

    // Spawn a thread to emulate the program
    std::thread::spawn({
        let pc = pc.clone();
        move || loop {
            **pc.get() += 1;
        }
    });

    rsx! {
        FlexRow {
            %pc
            "Program Counter: {pc}"
        }
    }
}
