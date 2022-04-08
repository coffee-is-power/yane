#[derive(Debug)]
pub struct Registers {
    pub a: u8,
    pub x: u8,
    pub y: u8,
    pub flags: u8,
    pub stack_pointer: u8,
    pub program_counter: u16,
}
impl Registers {
    pub fn new() -> Self {
        Self {
            a: 0,
            x: 0,
            y: 0,
            flags: 0x24,
            stack_pointer: 0xFD,
            program_counter: 0xFFFC,
        }
    }
    pub fn get_negative_flag(&self) -> bool {
        if (self.flags & 1) == 1 {
            true
        } else {
            false
        }
    }
    pub fn set_negative_flag(&mut self, bit: bool) {
        if bit {
            self.flags |= 1;
        } else {
            self.flags &= !1;
        }
    }
    pub fn get_overflow_flag(&self) -> bool {
        if ((self.flags & 0b10) >> 1) == 1 {
            true
        } else {
            false
        }
    }

    pub fn set_overflow_flag(&mut self, bit: bool) {
        if bit {
            self.flags |= 0b10;
        } else {
            self.flags &= !0b10;
        }
    }
    pub fn get_break_flag(&self) -> bool {
        if ((self.flags & 0b1000) >> 3) == 1 {
            true
        } else {
            false
        }
    }

    pub fn set_break_flag(&mut self, bit: bool) {
        if bit {
            self.flags |= 0b1000;
        } else {
            self.flags &= !0b1000;
        }
    }
    pub fn get_decimal_flag(&self) -> bool {
        if ((self.flags & 0b10000) >> 4) == 1 {
            true
        } else {
            false
        }
    }

    pub fn set_decimal_flag(&mut self, bit: bool) {
        if bit {
            self.flags |= 0b10000;
        } else {
            self.flags &= !0b10000;
        }
    }
    pub fn get_interrupt_disable_flag(&self) -> bool {
        if ((self.flags & 0b100000) >> 5) == 1 {
            true
        } else {
            false
        }
    }

    pub fn set_interrupt_disable_flag(&mut self, bit: bool) {
        if bit {
            self.flags |= 0b100000;
        } else {
            self.flags &= !0b100000;
        }
    }
    pub fn get_zero_flag(&self) -> bool {
        if ((self.flags & 0b1000000) >> 6) == 1 {
            true
        } else {
            false
        }
    }

    pub fn set_zero_flag(&mut self, bit: bool) {
        if bit {
            self.flags |= 0b1000000;
        } else {
            self.flags &= !0b1000000;
        }
    }
    pub fn get_carry_flag(&self) -> bool {
        if ((self.flags & 0b10000000) >> 7) == 1 {
            true
        } else {
            false
        }
    }

    pub fn set_carry_flag(&mut self, bit: bool) {
        if bit {
            self.flags |= 0b10000000;
        } else {
            self.flags &= !0b10000000;
        }
    }
}
