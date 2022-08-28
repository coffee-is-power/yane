use crate::mapper::Mapper;
use crate::mapper_0::Mapper0;
use std::fs::File;
use std::io::Read;

pub struct Cartridge {
    prg_memory: Vec<u8>,
    chr_memory: Vec<u8>,
    mapper: Box<dyn Mapper>,
}
#[repr(C, packed)]
#[derive(Debug)]
struct INesHeader {
    name: [u8; 4],
    prg_rom_chunks: u8,
    chr_rom_chunks: u8,
    mapper1: u8,
    mapper2: u8,
    prg_ram_size: u8,
    tv_system1: u8,
    tv_system2: u8,
    _unused: [u8; 5],
}
fn get_mapper_by_id(mapper_id: u8, prg_banks: u8, chr_banks: u8) -> Box<dyn Mapper> {
    match mapper_id {
        0 => Box::new(Mapper0::new(prg_banks, chr_banks)),
        _ => panic!("Unknown mapper: {}", mapper_id),
    }
}
impl Cartridge {
    pub fn from_rom(rom: Vec<u8>) -> Self {
        Self {
            chr_memory: vec![],
            prg_memory: rom,
            mapper: Box::new(Mapper0::new(1, 0)),
        }
    }
    pub fn from_file(path: &String) -> std::io::Result<Self> {
        let file = File::open(path)?;
        Ok(Self::from_read(Box::new(file)))
    }
    pub fn from_read(mut data: Box<dyn Read>) -> Self {
        let mut header_bytes = [0u8; std::mem::size_of::<INesHeader>()];
        data.read_exact(&mut header_bytes).unwrap();
        let header: INesHeader = unsafe { std::mem::transmute_copy(&header_bytes) };
        
        let mapper_id = ((header.mapper2 >> 4) << 4) | (header.mapper1 >> 4);
        let file_type = 1;
        
        match file_type {
            1 => {
                let prg_banks = header.prg_rom_chunks;
                let mut prg_memory = Vec::<u8>::new();
                prg_memory.resize((prg_banks as u32 * 0x4000) as usize, 0);
                data.read_exact(&mut prg_memory).unwrap();
                let chr_banks = header.chr_rom_chunks;
                let mut chr_memory = Vec::<u8>::new();
                chr_memory.resize((chr_banks as u32 * 0x4000) as usize, 0);
                data.read(&mut chr_memory).unwrap();
                let mapper: Box<dyn Mapper> = get_mapper_by_id(mapper_id, prg_banks, chr_banks);
                Self {
                    chr_memory,
                    prg_memory,
                    mapper
                }
            }
            _ => panic!("File type {} not supported!", file_type),
        }
    }
    pub fn cpu_read(&self, addr: u16) -> Option<u8> {
        if let Some(mapped_address)= (*self.mapper).cpu_map_read(addr) {
            Some(self.prg_memory[mapped_address as usize])
        } else {
            None
        }
    }
    pub fn cpu_write(&mut self, addr: u16, value: u8) -> bool {
        if let Some(mapped_address) = (*self.mapper).cpu_map_write(addr) {
            self.prg_memory[mapped_address as usize] = value;
            true
        } else {
            false
        }
    }
    pub fn ppu_read(&self, addr: u16) -> Option<u8> {
        if let Some(mapped_address)= (*self.mapper).ppu_map_read(addr) {
            Some(self.chr_memory[mapped_address as usize])
        } else {
            None
        }
    }
    pub fn ppu_write(&mut self, addr: u16, value: u8) -> bool {
        
        if let Some(mapped_address) = (*self.mapper).ppu_map_write(addr) {
            self.chr_memory[mapped_address as usize] = value;
            true
        } else {
            false
        }
    }
}
