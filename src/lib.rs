mod opcode;

use num_enum::TryFromPrimitive;
use opcode::Opcode;

pub struct CPU {
    pub reg_a: u8,
    pub status: u8,
    pub PC: u16,
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            reg_a: 0,
            status: 0,
            PC: 0,
        }
    }
    pub fn interpret(&mut self, program: &Vec<u8>) {
        // default PC is 0
        self.PC = 0;

        // fetch
        let mut fetch_byte = |data: &Vec<u8>| -> u8 {
            let byte = data[self.PC as usize];
            self.PC += 1;
            byte
        };
        loop {
            // fetch opcode
            let opcode = match Opcode::try_from(fetch_byte(program)) {
                Ok(code) => code,
                Err(e) => e,
            };
            match opcode {
                Opcode(i) => Opcode(i),
            }

            match opcode {
                Opcode::LDA => todo!(),
            }
        }
    }
}
