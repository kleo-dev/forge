pub mod instruction;
pub mod registry;

use std::fmt::Display;

use osui::state::{DependencyHandler, State};

use crate::emulator::{instruction::Instruction, registry::Registry};

pub struct Emulator {
    pub pc: State<usize>,
    pub instructions: Vec<Instruction>,
    pub registers: Registry,
    pub inst: State<Instruction>,
}

impl Emulator {
    pub fn run(&self) {
        loop {
            std::thread::sleep(std::time::Duration::from_millis(100));
            let mut pc = self.pc.get();
            if let Some(inst) = self.instructions.get(**pc) {
                **pc += 1;
                **self.inst.get() = inst.clone();
                match &inst {
                    // 1 | Halt
                    Instruction::Hlt => break,

                    // 2 | Registry set
                    Instruction::Reg(r, v) => self.registers.set(*r, *v),

                    // 3 | Add
                    Instruction::Add(rx, ry, r) => {
                        let x = self.registers.get(*rx);
                        let y = self.registers.get(*ry);
                        self.registers.set(*r, x + y);
                    }

                    // 3 | Subtract
                    Instruction::Sub(rx, ry, r) => {
                        let x = self.registers.get(*rx);
                        let y = self.registers.get(*ry);
                        self.registers.set(*r, x - y);
                    }

                    // 4 | Multiply
                    Instruction::Mul(rx, ry, r) => {
                        let x = self.registers.get(*rx);
                        let y = self.registers.get(*ry);
                        self.registers.set(*r, x * y);
                    }

                    // 5 | Divide
                    Instruction::Div(rx, ry, r) => {
                        let x = self.registers.get(*rx);
                        let y = self.registers.get(*ry);
                        self.registers.set(*r, x / y);
                    }
                }
            } else {
                break;
            }
        }
    }
}

impl Registry {
    pub fn get(&self, r: u8) -> usize {
        match self.0.get().get(r as usize) {
            Some(v) => *v,
            None => 0,
        }
    }

    pub fn set(&self, r: u8, v: usize) {
        if let Some(o) = self.0.get().get_mut(r as usize) {
            *o = v;
        }
    }
}

impl Display for Registry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "rx: {}, ry: {}\nrz: {}, ra: {}",
            self.get(0),
            self.get(1),
            self.get(2),
            self.get(3),
        )
    }
}

impl DependencyHandler for Registry {
    fn add(&self) {
        self.0.add()
    }

    fn check(&self) -> bool {
        self.0.check()
    }
}
