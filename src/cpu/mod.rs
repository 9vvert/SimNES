mod opcode;

use byteorder::{ByteOrder, LittleEndian};
use opcode::Opcode;

pub struct CPU {
    pub reg_a: u8,
    pub reg_x: u8,
    pub status: u8,
    pub PC: u16,
    memory: [u8; 0xFFFF],
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            reg_a: 0,
            reg_x: 0,
            status: 2,
            PC: 1,
            memory: [0; 0xFFFF],
        }
    }
    fn mem_read_byte(&self, addr: u16) -> u8 {
        return self.memory[addr as usize];
    }
    fn mem_write_byte(&mut self, addr: u16, data: u8) {
        self.memory[addr as usize] = data;
    }
    fn mem_read_word(&self, addr: u16) -> u16 {
        // NOTE:
        // 1. LittleEndian could be used to read data from slice
        // 2. change addr's type from u16 to usize , so that it can be used as index
        // 3. use "&self.memory" rather than "self.memory"
        let addr = addr as usize;
        return LittleEndian::read_u16(&self.memory[addr..addr + 2]);
    }
    fn mem_write_word(&mut self, addr: u16, data: u16) {
        let addr = addr as usize;
        return LittleEndian::write_u16(&mut self.memory[addr..addr + 2], data);
    }

    fn update_overflow_flag_on_inc(&mut self, prev: u8, temp: u8) {
        if prev > temp {
            self.status |= 0b0100_0000;
        } else {
            self.status &= 0b1011_1111;
        }
    }
    fn update_zero_signal_flag(&mut self, data: u8) {
        // set zero flag
        if data == 0 {
            self.status |= 0b0000_0010;
        } else {
            self.status &= 0b1111_1101;
        }
        // set signal flag
        if self.reg_a & 0b1000_0000 == 0 {
            self.status &= 0b0111_1111;
        } else {
            self.status |= 0b1000_0000;
        }
    }
    fn fetch_byte(&mut self) -> u8 {
        let byte = self.memory[self.PC as usize];
        self.PC += 1;
        byte
    }
    pub fn load_program(&mut self, program: &Vec<u8>) {
        self.memory[0x8000..0x8000 + program.len()].copy_from_slice(&program[..]);
        self.mem_write_word(0xFFFC, 0x8000);
    }
    pub fn init_program(&mut self) {
        self.reg_x = 0;
        self.reg_a = 0;
        self.status = 0;
        self.PC = self.mem_read_word(0xFFFC);
    }
    pub fn run_program(&mut self) {
        // default PC is 0

        loop {
            // fetch opcode
            let opcode = Opcode::from_u8(self.fetch_byte());

            match opcode {
                // NOTE:
                // 1. must cover all cases
                Opcode::LDA => {
                    let imm = self.fetch_byte();
                    self.reg_a = imm;
                    self.update_zero_signal_flag(self.reg_a);
                }
                Opcode::BRK => {
                    return;
                }
                Opcode::TAX => {
                    self.reg_x = self.reg_a;
                    self.update_zero_signal_flag(self.reg_x);
                }
                Opcode::INX => {
                    let prev = self.reg_x;
                    self.reg_x = self.reg_x.wrapping_add(1);
                    self.update_zero_signal_flag(self.reg_x);
                    self.update_overflow_flag_on_inc(prev, self.reg_x);
                }
                Opcode::Unknown => {
                    eprintln!("Unknown code: {} ", opcode as u8);
                    continue;
                }
            }
        }
    }
}
