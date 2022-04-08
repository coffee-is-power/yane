use rand::prelude::*;
#[derive(Debug)]
pub struct Memory {
    ram: [u8; 0x2000],
    rom: [u8; 0x7ffff]
}
impl Memory {
    pub fn new() -> Self {
        Self {
            ram: [0; 0x2000],
            rom: [0; 0x7ffff]
        }
    }
    pub fn write(&mut self, address: u16, data: u8) {
        if address < 0x2000 {
            self.ram[address as usize] = data;
        }
        else if address & 0b1000000000000000 == 0x8000 {
            eprintln!("ERROR: Writing to ROM!");
            // TODO: Remove this
            self.rom[(address & 0b0111111111111111) as usize] = data;
            return;
        }
        else {
            eprintln!("ERROR: Unmapped memory!");
        }
    }
    pub fn read(&self, address: u16) -> u8 {
        return if address < 0x2000 {
            self.ram[address as usize]
        } 
        else if address & 0b1000000000000000 == 0x8000 {
            self.rom[(address & 0b0111111111111111) as usize]
        } else {
            eprintln!("ERROR: Reading Unmapped memory!");
            let mut rnd = rand::thread_rng();
            /* Pretend like reading trash */
            rnd.gen()
        }
    }
}