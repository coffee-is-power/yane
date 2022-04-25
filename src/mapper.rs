pub trait Mapper {
    fn cpu_map_read(&self, addr: u16) -> (u16, bool);
    fn cpu_map_write(&self, addr: u16) -> (u16, bool);
    fn ppu_map_read(&self, addr: u16) -> (u16, bool);
    fn ppu_map_write(&self, addr: u16) -> (u16, bool);
}
