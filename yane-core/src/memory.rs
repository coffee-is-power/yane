use crate::cartridge::Cartridge;
use crate::ppu::PPU;
use std::{rc::Rc, cell::RefCell};

pub struct Memory {
    pub ram: [u8; 0x7ff],
    cartridge: Rc<RefCell<Cartridge>>,
    ppu: Rc<RefCell<PPU>>
}
impl Memory {
    pub fn new(cartridge: Rc<RefCell<Cartridge>>, ppu: Rc<RefCell<PPU>>) -> Self {
        Self {
            ram: [0; 0x7ff],
            cartridge,
            ppu
        }
    }
    pub fn cpu_write(&mut self, address: u16, data: u8) -> bool {
        if self.cartridge.borrow_mut()
            .cpu_write(address, data)
        {
            true
        } else if address < 0x2000 {
            self.ram[(address & 0x7ff) as usize] = data;
            true
        } else if address <= 0x3fff && address >= 0x2000 {
            
                self.ppu.borrow_mut()
            .cpu_write(address & 0x7, data);
            true
        } else {
            eprintln!("ERROR: Unmapped memory!");
            false
        }
    }
    pub fn cpu_read(&mut self, address: u16) -> Option<u8> {
        if let Some(value) = self.cartridge.borrow_mut().cpu_read(address) {
            Some(value)
        } else if address < 0x2000 {
            Some(self.ram[(address & 0x7ff) as usize])
        } else if address <= 0x3fff && address >= 0x2000 {
            Some(
                self.ppu.borrow_mut()
            .cpu_read(address & 0x7))
        } else {
            eprintln!("ERROR: Reading Unmapped memory!");
            None
        }
    }
    pub fn ppu_read(&self, address: u16) -> Option<u8> {
        if address < 0x1fff {
            self.cartridge.borrow_mut().ppu_read(address)
        } else {
            None
        }
    }
    pub fn ppu_write(&mut self, address: u16, data: u8) -> bool {
        if address < 0x1fff {
            self.cartridge.borrow_mut().ppu_write(address, data)
        } else {
            false
        }
    }
}
