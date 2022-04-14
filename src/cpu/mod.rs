pub mod registers;
#[cfg(test)]
mod tests;

use crate::memory::Memory;
use registers::Registers;
#[allow(dead_code)]
const CPU_CLOCK_DELAY: f64 = 5.58730074e-7;
#[cfg(not(test))]
#[allow(dead_code)]
fn sleep_cycles(cycles: u32) {
    std::thread::sleep(std::time::Duration::from_secs_f64(
        CPU_CLOCK_DELAY * cycles as f64,
    ));
}
#[cfg(test)]
#[allow(dead_code)]
fn sleep_cycles(_cycles: u32) {}
pub enum AddressingMode {
    Immediate,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    IndirectX,
    IndirectY,
    // NoneAddressing,
}

#[derive(Debug)]
pub struct CPU {
    pub registers: Registers,
    pub memory: Memory,
}
impl CPU {
    pub fn new() -> Self {
        Self {
            registers: Registers::new(),
            memory: Memory::new([0u8; 0x7fff]),
        }
    }

    pub fn with_rom(rom: [u8; 0x7fff]) -> Self {
        Self {
            registers: Registers::new(),
            memory: Memory::new(rom),
        }
    }
    fn read_u16(&self, addr: u16) -> u16 {
        self.read(addr) as u16 | ((self.read(addr + 1) as u16) << 8)
    }
    fn get_operand_address(&mut self, mode: AddressingMode) -> u16 {
        match mode {
            AddressingMode::Immediate => {
                self.registers.program_counter += 1;
                self.registers.program_counter - 1
            }

            AddressingMode::ZeroPage => {
                self.registers.program_counter += 1;
                self.read(self.registers.program_counter - 1) as u16
            }

            AddressingMode::Absolute => {
                self.registers.program_counter += 2;
                self.read_u16(self.registers.program_counter - 2)
            }

            AddressingMode::ZeroPageX => {
                self.registers.program_counter += 1;
                let pos = self.read(self.registers.program_counter - 1);
                let addr = pos.wrapping_add(self.registers.x) as u16;
                addr
            }
            AddressingMode::ZeroPageY => {
                self.registers.program_counter += 1;
                let pos = self.read(self.registers.program_counter - 1);
                let addr = pos.wrapping_add(self.registers.y) as u16;
                addr
            }

            AddressingMode::AbsoluteX => {
                self.registers.program_counter += 2;
                let base = self.read_u16(self.registers.program_counter - 2);
                let addr = base.wrapping_add(self.registers.x as u16);
                addr
            }
            AddressingMode::AbsoluteY => {
                self.registers.program_counter += 2;
                let base = self.read_u16(self.registers.program_counter - 2);
                let addr = base.wrapping_add(self.registers.y as u16);
                addr
            }

            AddressingMode::IndirectX => {
                self.registers.program_counter += 1;
                let base = self.read(self.registers.program_counter - 1);

                let ptr: u8 = (base as u8).wrapping_add(self.registers.x);
                let lo = self.read(ptr as u16);
                let hi = self.read(ptr.wrapping_add(1) as u16);
                (hi as u16) << 8 | (lo as u16)
            }
            AddressingMode::IndirectY => {
                self.registers.program_counter += 1;
                let base = self.read(self.registers.program_counter - 1);

                let lo = self.read(base as u16);
                let hi = self.read((base as u8).wrapping_add(1) as u16);
                let deref_base = (hi as u16) << 8 | (lo as u16);
                let deref = deref_base.wrapping_add(self.registers.y as u16);
                deref
            } /*AddressingMode::NoneAddressing => {
                  panic!("mode {:?} is not supported", mode);
              }*/
        }
    }
    pub fn write(&mut self, address: u16, data: u8) {
        println!("Write:\n  Address: {},\n  Data: {}", address, data);
        self.memory.write(address, data);
    }
    pub fn read(&self, address: u16) -> u8 {
        println!("Read Address {}", address);
        self.memory.read(address)
    }
    /** Executes the next instruction in the program counter */
    pub fn exec(&mut self) {
        let instruction = self.read(self.registers.program_counter);
        self.registers.program_counter += 1;
        match instruction {
            // NOP
            0xea => {
                sleep_cycles(2);
            }
            // ORA
            0x09 => {
                self.ora(AddressingMode::Immediate);
                sleep_cycles(2);
            }
            0x05 => {
                self.ora(AddressingMode::ZeroPage);
                sleep_cycles(3);
            }
            0x15 => {
                self.ora(AddressingMode::ZeroPageX);
                sleep_cycles(4);
            }

            0x0D => {
                self.ora(AddressingMode::Absolute);
                sleep_cycles(4);
            }
            0x1D => {
                self.ora(AddressingMode::AbsoluteX);
                sleep_cycles(4);
            }
            0x19 => {
                self.ora(AddressingMode::AbsoluteY);
                sleep_cycles(4);
            }
            0x01 => {
                self.ora(AddressingMode::IndirectX);
                sleep_cycles(4);
            }
            0x11 => {
                self.ora(AddressingMode::IndirectY);
                sleep_cycles(4);
            }
            // ADC
            0x69 => {
                self.adc(AddressingMode::Immediate);
                sleep_cycles(2);
            }
            0x65 => {
                self.adc(AddressingMode::ZeroPage);
                sleep_cycles(3);
            }
            0x75 => {
                self.adc(AddressingMode::ZeroPageX);
                sleep_cycles(4);
            }
            0x6D => {
                self.adc(AddressingMode::Absolute);
                sleep_cycles(4);
            }
            0x7D => {
                self.adc(AddressingMode::AbsoluteX);
                sleep_cycles(4);
            }
            0x79 => {
                self.adc(AddressingMode::AbsoluteY);
                sleep_cycles(4);
            }
            0x61 => {
                self.adc(AddressingMode::IndirectX);
                sleep_cycles(6);
            }
            0x71 => {
                self.adc(AddressingMode::IndirectY);
                sleep_cycles(5);
            }
            // LDA
            0xA9 => {
                self.lda(AddressingMode::Immediate);
                sleep_cycles(2);
            }
            0xA5 => {
                self.lda(AddressingMode::ZeroPage);
                sleep_cycles(3);
            }
            0xB5 => {
                self.lda(AddressingMode::ZeroPageX);
                sleep_cycles(4);
            }

            0xAD => {
                self.lda(AddressingMode::Absolute);
                sleep_cycles(4);
            }

            0xBD => {
                self.lda(AddressingMode::AbsoluteX);
                sleep_cycles(4);
            }
            0xB9 => {
                self.lda(AddressingMode::AbsoluteY);
                sleep_cycles(4);
            }
            0xA1 => {
                self.lda(AddressingMode::IndirectX);
                sleep_cycles(6);
            }
            0xB1 => {
                self.lda(AddressingMode::IndirectY);
                sleep_cycles(5);
            }
            // LDX
            0xA2 => {
                self.ldx(AddressingMode::Immediate);
                sleep_cycles(2);
            }
            0xA6 => {
                self.ldx(AddressingMode::ZeroPage);
                sleep_cycles(3);
            }

            0xB6 => {
                self.ldx(AddressingMode::ZeroPageY);
                sleep_cycles(4);
            }

            0xAE => {
                self.ldx(AddressingMode::Absolute);
                sleep_cycles(4);
            }

            0xBE => {
                self.ldx(AddressingMode::AbsoluteY);
                sleep_cycles(4);
            }

            // LDY
            0xA0 => {
                self.ldy(AddressingMode::Immediate);
                sleep_cycles(2);
            }
            0xA4 => {
                self.ldy(AddressingMode::ZeroPage);
                sleep_cycles(3);
            }

            0xB4 => {
                self.ldy(AddressingMode::ZeroPageX);
                sleep_cycles(4);
            }

            0xAC => {
                self.ldy(AddressingMode::Absolute);
                sleep_cycles(4);
            }

            0xBC => {
                self.ldy(AddressingMode::AbsoluteX);
                sleep_cycles(4);
            }
            // AND
            0x29 => {
                self.and(AddressingMode::Immediate);
                sleep_cycles(2);
            }
            0x25 => {
                self.and(AddressingMode::ZeroPage);
                sleep_cycles(3);
            }
            0x35 => {
                self.and(AddressingMode::ZeroPageX);
                sleep_cycles(4);
            }

            0x2D => {
                self.and(AddressingMode::Absolute);
                sleep_cycles(4);
            }
            0x3D => {
                self.and(AddressingMode::AbsoluteX);
                sleep_cycles(4);
            }
            0x39 => {
                self.and(AddressingMode::AbsoluteY);
                sleep_cycles(4);
            }
            0x21 => {
                self.and(AddressingMode::IndirectX);
                sleep_cycles(6);
            }
            0x31 => {
                self.and(AddressingMode::IndirectY);
                sleep_cycles(5);
            }
            _ => unimplemented!("{:#02x} opcode is not implemented or illegal!", instruction),
        }
    }
    fn lda(&mut self, mode: AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.read(addr);
        self.registers.a = value;
    }

    fn ldx(&mut self, mode: AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.read(addr);
        self.registers.x = value;
    }

    fn ldy(&mut self, mode: AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.read(addr);
        self.registers.y = value;
    }
    fn adc(&mut self, mode: AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.read(addr);
        let (result, carry) = self.registers.a.overflowing_add(
            value
                + if self.registers.get_carry_flag() {
                    1
                } else {
                    0
                },
        );
        self.registers.a = result;
        self.registers.set_carry_flag(carry);
        self.registers.set_zero_flag(result == 0);
        self.registers.set_overflow_flag(carry);
        self.registers.set_negative_flag(result >= 0x80);
    }

    fn ora(&mut self, mode: AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.read(addr);
        self.registers.a |= value;
    }

    fn and(&mut self, mode: AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.read(addr);
        self.registers.a &= value;
    }
    pub fn init(&mut self) {
        let low_byte = self.read(0xFFFC);
        let high_byte = self.read(0xFFFD);
        let pc: u16 = (low_byte as u16) | ((high_byte as u16) << 8);
        self.registers.program_counter = pc;
    }
}
