use crate::cartridge::Cartridge;
use std::borrow::BorrowMut;
use std::rc::Rc;

pub struct Memory {
    pub ram: [u8; 0x2000],
    cartridge: Rc<Cartridge>,
}
impl Memory {
    pub fn new(cartridge: Rc<Cartridge>) -> Self {
        Self {
            ram: [0; 0x2000],
            cartridge,
        }
    }
    pub fn cpu_write(&mut self, address: u16, data: u8) {
        if address < 0x2000 {
            self.ram[address as usize] = data;
        } else if address & 0b1000000000000000 == 0x8000 {
            Rc::get_mut(&mut self.cartridge)
                .unwrap()
                .cpu_write(address, data);
        } else {
            eprintln!("ERROR: Unmapped memory!");
        }
    }
    pub fn cpu_read(&self, address: u16) -> u8 {
        if address < 0x2000 {
            self.ram[address as usize]
        } else if address & 0b1000000000000000 == 0x8000 {
            self.cartridge.cpu_read(address)
        } else {
            eprintln!("ERROR: Reading Unmapped memory!");
            0x00
        }
    }
    pub fn ppu_read(&self, address: u16) -> u8 {
        if address < 0x1fff {
            self.cartridge.ppu_read(address)
        } else {
            0
        }
    }
    pub fn ppu_write(&mut self, address: u16, data: u8) {
        if address < 0x1fff {
            Rc::get_mut(&mut self.cartridge)
                .unwrap()
                .ppu_write(address, data)
        }
    }
}
