use crate::cartridge::Cartridge;
use std::borrow::BorrowMut;
use std::rc::Rc;

pub struct Memory {
    pub ram: [u8; 0x7ff],
    cartridge: Rc<Cartridge>,
}
impl Memory {
    pub fn new(cartridge: Rc<Cartridge>) -> Self {
        Self {
            ram: [0; 0x7ff],
            cartridge,
        }
    }
    pub fn cpu_write(&mut self, address: u16, data: u8) -> bool {
        if Rc::get_mut(&mut self.cartridge)
            .unwrap()
            .cpu_write(address, data)
        {
            true
        } else if address < 0x2000 {
            self.ram[(address & 0x7ff) as usize] = data;
            true
        } else {
            eprintln!("ERROR: Unmapped memory!");
            false
        }
    }
    pub fn cpu_read(&self, address: u16) -> (u8, bool) {
        let (value, interested) = self.cartridge.cpu_read(address);
        if interested {
            (value, interested)
        } else if address < 0x2000 {
            (self.ram[(address & 0x7ff) as usize], true)
        } else {
            eprintln!("ERROR: Reading Unmapped memory!");
            (0, false)
        }
    }
    pub fn ppu_read(&self, address: u16) -> (u8, bool) {
        if address < 0x1fff {
            self.cartridge.ppu_read(address)
        } else {
            (0, false)
        }
    }
    pub fn ppu_write(&mut self, address: u16, data: u8) -> bool {
        if address < 0x1fff {
            Rc::get_mut(&mut self.cartridge)
                .unwrap()
                .ppu_write(address, data)
        } else {
            false
        }
    }
}
