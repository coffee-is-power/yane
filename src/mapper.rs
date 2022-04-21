pub trait Mapper {
    fn cpu_map_read(&self, addr: u16) -> u16;
    fn cpu_map_write(&mut self, addr: u16) -> u16;
}
