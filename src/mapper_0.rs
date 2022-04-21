use crate::mapper::Mapper;

pub struct Mapper0 {
    prg_banks: u8,
    chr_banks: u8,
}

impl Mapper for Mapper0 {
    fn cpu_map_read(&self, addr: u16) -> u16 {
        addr & (if self.prg_banks > 1 { 0x7fff } else { 0x3fff })
    }

    fn cpu_map_write(&mut self, addr: u16) -> u16 {
        addr
    }
}
impl Mapper0 {
    pub fn new(prg_banks: u8, chr_banks: u8) -> Self {
        Self {
            prg_banks,
            chr_banks,
        }
    }
}
