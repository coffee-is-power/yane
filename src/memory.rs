use crate::cartridge::Cartridge;
#[derive(Debug)]
pub struct Memory {
    pub ram: [u8; 0x2000],
    cartridge: *mut Cartridge,
}
impl Memory {
    pub fn new(cartridge: &mut Cartridge) -> Self {
        Self {
            ram: [0; 0x2000],
            cartridge,
        }
    }
    pub fn mut_cartridge(&mut self) -> &mut Cartridge {
        unsafe { &mut *self.cartridge }
    }
    pub fn cartridge(&self) -> &Cartridge {
        unsafe { &*self.cartridge }
    }
    pub fn cpu_write(&mut self, address: u16, data: u8) {
        if address < 0x2000 {
            self.ram[address as usize] = data;
        } else if address & 0b1000000000000000 == 0x8000 {
            self.mut_cartridge().cpu_write(address, data);
        } else {
            eprintln!("ERROR: Unmapped memory!");
        }
    }
    pub fn cpu_read(&self, address: u16) -> u8 {
        return if address < 0x2000 {
            self.ram[address as usize]
        } else if address & 0b1000000000000000 == 0x8000 {
            self.cartridge().cpu_read(address)
        } else {
            eprintln!("ERROR: Reading Unmapped memory!");
            0x00
        };
    }
}
