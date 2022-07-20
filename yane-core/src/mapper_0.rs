use crate::mapper::Mapper;

pub struct Mapper0 {
    prg_banks: u8,
    chr_banks: u8,
}

impl Mapper for Mapper0 {
    fn cpu_map_read(&self, addr: u16) -> Option<u16> {
        if addr >= 0x8000 {
            let mask: u16;
            if self.prg_banks > 1 {
                mask = 0x7fff;
            } else {
                mask = 0x3FFF;
            }
            Some(addr & mask)
        } else {
            None
        }
    }

    fn cpu_map_write(&self, addr: u16) -> Option<u16> {
        if addr >= 0x8000 {
            let mask: u16;
            if self.prg_banks > 1 {
                mask = 0x7fff;
            } else {
                mask = 0x3FFF;
            }
            Some(addr & mask)
        } else {
            None
        }
    }
    fn ppu_map_read(&self, addr: u16) -> Option<u16> {
        if addr < 0x2000 {
            Some(addr)
        } else {
            None
        }
    }
    fn ppu_map_write(&self, addr: u16) -> Option<u16> {
        if addr < 0x2000 && self.chr_banks == 0 {
            Some(addr)
        } else {
            None
        }
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
