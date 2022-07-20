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
            // The original hardware doesn't really initialize this register, but i'm gonna set to 0x24
            // The software will change this on runtime
            // So it doesnt really matter
            stack_pointer: 0x24,
            program_counter: 0xFFFC,
        }
    }
    pub fn set_flags_from_byte(&mut self, flags: u8) {
        self.negative = (flags & 1) == 1;
        self.overflow = (flags & 0b10) == 0b10;
        self.interrupt_disable = (flags & 0b100000) == 0b100000;
        self.zero = (flags & 0b1000000) == 0b1000000;
        self.carry = (flags & 0b10000000) == 0b10000000;
    }
    pub fn get_flags(&self) -> u8 {
        let mut result = 0u8;
        if self.negative {
            result |= 1;
        }
        if self.overflow {
            result |= 0b10;
        }
        if self.interrupt_disable {
            result |= 0b100000;
        }
        if self.zero {
            result |= 0b1000000;
        }
        if self.carry {
            result |= 0b10000000;
        }
        result
    }
}
