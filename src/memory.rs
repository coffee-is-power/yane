#[derive(Debug)]
pub struct Memory {
    pub ram: [u8; 0x2000],
    pub rom: [u8; 0x7fff],
}
impl Memory {
    pub fn new(rom: [u8; 0x7fff]) -> Self {
        Self {
            ram: [0; 0x2000],
            rom,
        }
    }

    pub fn write(&mut self, address: u16, data: u8) {
        if address < 0x2000 {
            self.ram[address as usize] = data;
        } else if address & 0b1000000000000000 == 0x8000 {
            eprintln!("ERROR: Writing to ROM!");
            return;
        } else {
            eprintln!("ERROR: Unmapped memory!");
        }
    }
    pub fn read(&self, address: u16) -> u8 {
        return if address < 0x2000 {
            self.ram[address as usize]
        } else if address & 0b1000000000000000 == 0x8000 {
            self.rom[(address & 0b0111111111111111) as usize]
        } else {
            eprintln!("ERROR: Reading Unmapped memory!");
            0xFF
        };
    }
}
