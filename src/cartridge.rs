use crate::mapper::Mapper;
use crate::mapper_0::Mapper0;
use std::fs::File;
use std::io::Read;
use std::os::unix::fs::MetadataExt;

pub struct Cartridge {
    prg_memory: Vec<u8>,
    chr_memory: Vec<u8>,
    mapper_id: u8,
    prg_banks: u8,
    chr_banks: u8,
    header: INesHeader,
    mapper: Box<dyn Mapper>,
}
#[repr(C, packed)]
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
    pub fn from_file(path: &String) -> std::io::Result<Self> {
        let mut file = File::open(path)?;
        let mut data = [0u8; std::mem::size_of::<INesHeader>()];
        file.read_exact(&mut data)?;
        let header: INesHeader = unsafe { std::mem::transmute_copy(&data) };
        let mapper_id = ((header.mapper2 >> 4) << 4) | (header.mapper1 >> 4);
        let file_type = 1;

        match file_type {
            1 => {
                let prg_banks = header.prg_rom_chunks;
                let mut prg_memory = Vec::<u8>::new();
                prg_memory.resize((prg_banks as u32 * 0x4000) as usize, 0);
                file.read_exact(prg_memory.as_mut_slice())?;

                let chr_banks = header.chr_rom_chunks;
                let mut chr_memory = Vec::<u8>::new();
                prg_memory.resize((chr_banks as u32 * 0x4000) as usize, 0);
                file.read_exact(chr_memory.as_mut_slice())?;
                let mapper: Box<dyn Mapper> = get_mapper_by_id(mapper_id, prg_banks, chr_banks);
                Ok(Self {
                    chr_banks,
                    chr_memory,
                    prg_memory,
                    prg_banks,
                    mapper_id,
                    header,
                    mapper,
                })
            }
            _ => panic!("File type {} not supported!", file_type),
        }
    }
    pub fn cpu_read(&self, addr: u16) -> u8 {
        let mapped_address = (*self.mapper).cpu_map_read(addr);
        self.prg_memory[mapped_address as usize]
    }
    pub fn cpu_write(&mut self, addr: u16, value: u8) {
        let mapped_address = (*self.mapper).cpu_map_write(addr);
        self.prg_memory[mapped_address as usize] = value;
    }
}
