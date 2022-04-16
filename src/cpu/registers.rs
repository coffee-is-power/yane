#[derive(Debug)]
pub struct Registers {
    pub a: u8,
    pub x: u8,
    pub y: u8,
    pub stack_pointer: u8,
    pub program_counter: u16,
    pub negative: bool,
    pub overflow: bool,
    pub interrupt_disable: bool,
    pub zero: bool,
    pub carry: bool,
}
impl Registers {
    pub fn new() -> Self {
        Self {
            a: 0,
            x: 0,
            y: 0,
            negative: false,
            overflow: false,
            interrupt_disable: true,
            zero: false,
            carry: false,
            stack_pointer: 0xFD,
            program_counter: 0xFFFC,
        }
    }
}
