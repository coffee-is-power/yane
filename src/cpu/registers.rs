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
            // The original hardware doesn't really initialize this register, but i'm gonna set to 0xFF
            // The software will change this on runtime
            // So it doesnt really matter
            stack_pointer: 0xFF,
            program_counter: 0xFFFC,
        }
    }
}
