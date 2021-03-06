pub mod registers;
#[cfg(test)]
mod tests;

use crate::memory::Memory;
use registers::Registers;
use std::{rc::Rc, sync::{Arc, Mutex}};

#[derive(Debug)]
pub enum AddressingMode {
    Immediate,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    Indirect,
    IndirectX,
    IndirectY,
    NoneAddressing,
}

pub struct CPU {
    pub registers: Registers,
    pub memory: Rc<Memory>,
    pub remaining_cycles: u32,
}
impl CPU {
    pub fn new(memory: Rc<Memory>) -> Self {
        Self {
            registers: Registers::new(),
            memory,
            remaining_cycles: 0,
        }
    }
    pub fn clock(&mut self) {
        if self.remaining_cycles > 0 {
            self.remaining_cycles -= 1;
        } else {
            self.exec();
        }
    }
    pub fn read_u16(&mut self, addr: u16) -> u16 {
        self.read(addr) as u16 | ((self.read(addr + 1) as u16) << 8)
    }
    fn get_operand_address(&mut self, mode: AddressingMode) -> u16 {
        match mode {
            AddressingMode::Indirect => {
                self.registers.program_counter += 2;
                let ptr_to_ptr = self.read_u16(self.registers.program_counter - 2);
                self.read_u16(ptr_to_ptr)
            }
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
            }
            AddressingMode::NoneAddressing => {
                panic!("mode {:?} is not supported", mode);
            }
        }
    }
    pub fn write(&mut self, address: u16, data: u8) {
        println!("Write:\n  Address: {},\n  Data: {}", address, data);

        Rc::get_mut(&mut self.memory)
            .unwrap()
            .cpu_write(address, data);
    }
    pub fn read(&mut self, address: u16) -> u8 {
        let r = Rc::get_mut(&mut self.memory)
        .unwrap().cpu_read(address).or_else(|| Some(0)).unwrap();
        println!("Read Address {:#02x}: {:#02x}", address, r);
        r
    }
    /** Executes the next instruction in the program counter */
    pub fn exec(&mut self) {
        let instruction = self.read(self.registers.program_counter);
        self.registers.program_counter += 1;
        match instruction {
            // NOP
            0xea => {
                self.remaining_cycles = 2;
            }
            // ORA
            0x09 => {
                self.ora(AddressingMode::Immediate);
                self.remaining_cycles = 2;
            }
            0x05 => {
                self.ora(AddressingMode::ZeroPage);
                self.remaining_cycles = 3;
            }
            0x15 => {
                self.ora(AddressingMode::ZeroPageX);
                self.remaining_cycles = 4;
            }

            0x0D => {
                self.ora(AddressingMode::Absolute);
                self.remaining_cycles = 4;
            }
            0x1D => {
                self.ora(AddressingMode::AbsoluteX);
                self.remaining_cycles = 4;
            }
            0x19 => {
                self.ora(AddressingMode::AbsoluteY);
                self.remaining_cycles = 4;
            }
            0x01 => {
                self.ora(AddressingMode::IndirectX);
                self.remaining_cycles = 4;
            }
            0x11 => {
                self.ora(AddressingMode::IndirectY);
                self.remaining_cycles = 4;
            }
            // ADC
            0x69 => {
                self.adc(AddressingMode::Immediate);
                self.remaining_cycles = 2;
            }
            0x65 => {
                self.adc(AddressingMode::ZeroPage);
                self.remaining_cycles = 3;
            }
            0x75 => {
                self.adc(AddressingMode::ZeroPageX);
                self.remaining_cycles = 4;
            }
            0x6D => {
                self.adc(AddressingMode::Absolute);
                self.remaining_cycles = 4;
            }
            0x7D => {
                self.adc(AddressingMode::AbsoluteX);
                self.remaining_cycles = 4;
            }
            0x79 => {
                self.adc(AddressingMode::AbsoluteY);
                self.remaining_cycles = 4;
            }
            0x61 => {
                self.adc(AddressingMode::IndirectX);
                self.remaining_cycles = 6;
            }
            0x71 => {
                self.adc(AddressingMode::IndirectY);
                self.remaining_cycles = 5;
            }
            // LDA
            0xA9 => {
                self.lda(AddressingMode::Immediate);
                self.remaining_cycles = 2;
            }
            0xA5 => {
                self.lda(AddressingMode::ZeroPage);
                self.remaining_cycles = 3;
            }
            0xB5 => {
                self.lda(AddressingMode::ZeroPageX);
                self.remaining_cycles = 4;
            }

            0xAD => {
                self.lda(AddressingMode::Absolute);
                self.remaining_cycles = 4;
            }

            0xBD => {
                self.lda(AddressingMode::AbsoluteX);
                self.remaining_cycles = 4;
            }
            0xB9 => {
                self.lda(AddressingMode::AbsoluteY);
                self.remaining_cycles = 4;
            }
            0xA1 => {
                self.lda(AddressingMode::IndirectX);
                self.remaining_cycles = 6;
            }
            0xB1 => {
                self.lda(AddressingMode::IndirectY);
                self.remaining_cycles = 5;
            }
            // LDX
            0xA2 => {
                self.ldx(AddressingMode::Immediate);
                self.remaining_cycles = 2;
            }
            0xA6 => {
                self.ldx(AddressingMode::ZeroPage);
                self.remaining_cycles = 3;
            }

            0xB6 => {
                self.ldx(AddressingMode::ZeroPageY);
                self.remaining_cycles = 4;
            }

            0xAE => {
                self.ldx(AddressingMode::Absolute);
                self.remaining_cycles = 4;
            }

            0xBE => {
                self.ldx(AddressingMode::AbsoluteY);
                self.remaining_cycles = 4;
            }

            // LDY
            0xA0 => {
                self.ldy(AddressingMode::Immediate);
                self.remaining_cycles = 2;
            }
            0xA4 => {
                self.ldy(AddressingMode::ZeroPage);
                self.remaining_cycles = 3;
            }

            0xB4 => {
                self.ldy(AddressingMode::ZeroPageX);
                self.remaining_cycles = 4;
            }

            0xAC => {
                self.ldy(AddressingMode::Absolute);
                self.remaining_cycles = 4;
            }

            0xBC => {
                self.ldy(AddressingMode::AbsoluteX);
                self.remaining_cycles = 4;
            }
            // AND
            0x29 => {
                self.and(AddressingMode::Immediate);
                self.remaining_cycles = 2;
            }
            0x25 => {
                self.and(AddressingMode::ZeroPage);
                self.remaining_cycles = 3;
            }
            0x35 => {
                self.and(AddressingMode::ZeroPageX);
                self.remaining_cycles = 4;
            }

            0x2D => {
                self.and(AddressingMode::Absolute);
                self.remaining_cycles = 4;
            }
            0x3D => {
                self.and(AddressingMode::AbsoluteX);
                self.remaining_cycles = 4;
            }
            0x39 => {
                self.and(AddressingMode::AbsoluteY);
                self.remaining_cycles = 4;
            }
            0x21 => {
                self.and(AddressingMode::IndirectX);
                self.remaining_cycles = 6;
            }
            0x31 => {
                self.and(AddressingMode::IndirectY);
                self.remaining_cycles = 5;
            }
            // EOR
            0x49 => {
                self.xor(AddressingMode::Immediate);
                self.remaining_cycles = 2;
            }
            0x45 => {
                self.xor(AddressingMode::ZeroPage);
                self.remaining_cycles = 3;
            }
            0x55 => {
                self.xor(AddressingMode::ZeroPageX);
                self.remaining_cycles = 4;
            }

            0x4D => {
                self.xor(AddressingMode::Absolute);
                self.remaining_cycles = 4;
            }
            0x5D => {
                self.xor(AddressingMode::AbsoluteX);
                self.remaining_cycles = 4;
            }
            0x59 => {
                self.xor(AddressingMode::AbsoluteY);
                self.remaining_cycles = 4;
            }
            0x41 => {
                self.xor(AddressingMode::IndirectX);
                self.remaining_cycles = 6;
            }
            0x51 => {
                self.xor(AddressingMode::IndirectY);
                self.remaining_cycles = 5;
            }
            // LSR
            0x4A => {
                self.lsr(AddressingMode::NoneAddressing);
                self.remaining_cycles = 2;
            }
            0x46 => {
                self.lsr(AddressingMode::ZeroPage);
                self.remaining_cycles = 5;
            }

            0x56 => {
                self.lsr(AddressingMode::ZeroPageX);
                self.remaining_cycles = 6;
            }
            0x4E => {
                self.lsr(AddressingMode::Absolute);
                self.remaining_cycles = 6;
            }
            0x5E => {
                self.lsr(AddressingMode::AbsoluteX);
                self.remaining_cycles = 7;
            }
            // ASL
            0x0A => {
                self.asl(AddressingMode::NoneAddressing);
                self.remaining_cycles = 2;
            }
            0x06 => {
                self.asl(AddressingMode::ZeroPage);
                self.remaining_cycles = 5;
            }

            0x16 => {
                self.asl(AddressingMode::ZeroPageX);
                self.remaining_cycles = 6;
            }
            0x0E => {
                self.asl(AddressingMode::Absolute);
                self.remaining_cycles = 6;
            }
            0x1E => {
                self.asl(AddressingMode::AbsoluteX);
                self.remaining_cycles = 7;
            }
            // BCC
            0x90 => {
                self.bcc();
                self.remaining_cycles = 2;
            }

            // BCS
            0xB0 => {
                self.bcs();
                self.remaining_cycles = 2;
            }
            // BVC
            0x50 => {
                self.bvc();
                self.remaining_cycles = 2;
            }
            // BVS
            0x70 => {
                self.bvs();
                self.remaining_cycles = 2;
            }
            // BEQ
            0xF0 => {
                self.beq();
                self.remaining_cycles = 2;
            }
            // BNE
            0xD0 => {
                self.bne();
                self.remaining_cycles = 2;
            }
            // BPL
            0x10 => {
                self.bpl();
                self.remaining_cycles = 2;
            }
            // BMI
            0x30 => {
                self.bmi();
                self.remaining_cycles = 2;
            }
            // CLI
            0x58 => {
                self.cli();
                self.remaining_cycles = 2;
            }
            // SEI
            0x78 => {
                self.sei();
                self.remaining_cycles = 2;
            }
            // JSR
            0x20 => {
                self.jsr();
                self.remaining_cycles = 6;
            }
            // RTS
            0x60 => {
                self.rts();
                self.remaining_cycles = 6;
            }
            // CLC
            0x18 => {
                self.clc();
                self.remaining_cycles = 2;
            }
            // SEC
            0x38 => {
                self.sec();
                self.remaining_cycles = 2;
            }
            // CLV
            0xB8 => {
                self.clv();
                self.remaining_cycles = 2;
            }
            // PHA
            0x48 => {
                self.pha();
                self.remaining_cycles = 3;
            }
            // PLA
            0x68 => {
                self.pla();
                self.remaining_cycles = 4;
            }
            // PHP
            0x08 => {
                self.php();
                self.remaining_cycles = 3;
            }
            // PLP
            0x28 => {
                self.plp();
                self.remaining_cycles = 3;
            }
            // TXS
            0x9A => {
                self.txs();
                self.remaining_cycles = 2;
            }
            // TSX
            0xBA => {
                self.tsx();
                self.remaining_cycles = 2;
            }
            // BIT zp
            0x24 => {
                self.bit(AddressingMode::ZeroPage);
                self.remaining_cycles = 3;
            }
            // BIT abs
            0x2C => {
                self.bit(AddressingMode::Absolute);
                self.remaining_cycles = 4;
            }
            // TAX
            0xAA => {
                self.tax();
                self.remaining_cycles = 2;
            }
            // TXA
            0x8A => {
                self.txa();
                self.remaining_cycles = 2;
            }
            // TAY
            0xA8 => {
                self.tay();
                self.remaining_cycles = 2;
            }
            // TYA
            0x98 => {
                self.tya();
                self.remaining_cycles = 2;
            }
            // STA zp
            0x85 => {
                self.sta(AddressingMode::ZeroPage);
                self.remaining_cycles = 3;
            }
            // STA zp + x
            0x95 => {
                self.sta(AddressingMode::ZeroPageX);
                self.remaining_cycles = 4;
            }
            // STA abs
            0x8D => {
                self.sta(AddressingMode::Absolute);
                self.remaining_cycles = 4;
            }
            // STA abs + x
            0x9D => {
                self.sta(AddressingMode::AbsoluteX);
                self.remaining_cycles = 5;
            }
            // STA abs + y
            0x99 => {
                self.sta(AddressingMode::AbsoluteY);
                self.remaining_cycles = 5;
            }
            // STA indirect x
            0x81 => {
                self.sta(AddressingMode::IndirectX);
                self.remaining_cycles = 6;
            }
            // STA indirect y
            0x91 => {
                self.sta(AddressingMode::IndirectY);
                self.remaining_cycles = 6;
            }

            // STX zp
            0x86 => {
                self.stx(AddressingMode::ZeroPage);
                self.remaining_cycles = 3;
            }
            // STX zp + y
            0x96 => {
                self.stx(AddressingMode::ZeroPageY);
                self.remaining_cycles = 4;
            }
            // STX abs
            0x8E => {
                self.stx(AddressingMode::Absolute);
                self.remaining_cycles = 4;
            }
            // STy zp
            0x84 => {
                self.sty(AddressingMode::ZeroPage);
                self.remaining_cycles = 3;
            }
            // STY zp + x
            0x94 => {
                self.sty(AddressingMode::ZeroPageX);
                self.remaining_cycles = 4;
            }
            // STY abs
            0x8C => {
                self.sty(AddressingMode::Absolute);
                self.remaining_cycles = 4;
            }
            // BRK
            0 => {
                self.irq();
                self.remaining_cycles = 7;
            }
            // CMP imm
            0xC9 => {
                self.cmp(AddressingMode::Immediate);
                self.remaining_cycles = 2;
            }
            // CMP zp
            0xC5 => {
                self.cmp(AddressingMode::ZeroPage);
                self.remaining_cycles = 3;
            }
            // CMP zp x
            0xD5 => {
                self.cmp(AddressingMode::ZeroPageX);
                self.remaining_cycles = 4;
            }
            // CMP abs
            0xCD => {
                self.cmp(AddressingMode::Absolute);
                self.remaining_cycles = 4;
            }
            // CMP abs X
            0xDD => {
                self.cmp(AddressingMode::AbsoluteX);
                self.remaining_cycles = 4;
            }
            // CMP abs Y
            0xD9 => {
                self.cmp(AddressingMode::AbsoluteY);
                self.remaining_cycles = 4;
            }
            // CMP indirect X
            0xC1 => {
                self.cmp(AddressingMode::IndirectX);
                self.remaining_cycles = 6;
            }
            // CMP indirect Y
            0xD1 => {
                self.cmp(AddressingMode::IndirectY);
                self.remaining_cycles = 5;
            }
            //CPX imm
            0xE0 => {
                self.cpx(AddressingMode::Immediate);
                self.remaining_cycles = 2;
            }
            //CPX zp
            0xE4 => {
                self.cpx(AddressingMode::ZeroPage);
                self.remaining_cycles = 3;
            }
            //CPX abs
            0xEC => {
                self.cpx(AddressingMode::Absolute);
                self.remaining_cycles = 4;
            }
            //CPY imm
            0xC0 => {
                self.cpy(AddressingMode::Immediate);
                self.remaining_cycles = 2;
            }
            //CPY zp
            0xC4 => {
                self.cpy(AddressingMode::ZeroPage);
                self.remaining_cycles = 3;
            }
            //CPY abs
            0xCC => {
                self.cpy(AddressingMode::Absolute);
                self.remaining_cycles = 4;
            }
            // DEC zp
            0xC6 => {
                self.dec(AddressingMode::ZeroPage);
                self.remaining_cycles = 5;
            }
            // DEC zp x
            0xD6 => {
                self.dec(AddressingMode::ZeroPageX);
                self.remaining_cycles = 5;
            }
            // DEC abs
            0xCE => {
                self.dec(AddressingMode::Absolute);
                self.remaining_cycles = 5;
            }
            // DEC abs x
            0xDE => {
                self.dec(AddressingMode::AbsoluteX);
                self.remaining_cycles = 5;
            }

            // INC zp
            0xE6 => {
                self.inc(AddressingMode::ZeroPage);
                self.remaining_cycles = 5;
            }
            // INC zp x
            0xF6 => {
                self.inc(AddressingMode::ZeroPageX);
                self.remaining_cycles = 6;
            }
            // INC abs
            0xEE => {
                self.inc(AddressingMode::Absolute);
                self.remaining_cycles = 6;
            }
            // INC abs x
            0xFE => {
                self.inc(AddressingMode::AbsoluteX);
                self.remaining_cycles = 7;
            }

            //JMP abs
            0x4C => {
                self.jmp(AddressingMode::Absolute);
                self.remaining_cycles = 3;
            }
            //JMP indirect
            0x6C => {
                self.jmp(AddressingMode::Indirect);
                self.remaining_cycles = 3;
            }
            // SBC imm
            0xE9 => {
                self.sbc(AddressingMode::Immediate);
                self.remaining_cycles = 2;
            }
            // SBC zp
            0xE5 => {
                self.sbc(AddressingMode::ZeroPage);
                self.remaining_cycles = 3;
            }

            // SBC zp x
            0xF5 => {
                self.sbc(AddressingMode::ZeroPageX);
                self.remaining_cycles = 4;
            }
            // SBC abs
            0xED => {
                self.sbc(AddressingMode::Absolute);
                self.remaining_cycles = 4;
            }

            // SBC abs X
            0xFD => {
                self.sbc(AddressingMode::AbsoluteX);
                self.remaining_cycles = 4;
            }
            // SBC abs Y
            0xF9 => {
                self.sbc(AddressingMode::AbsoluteY);
                self.remaining_cycles = 4;
            }
            // SBC Indirect X
            0xE1 => {
                self.sbc(AddressingMode::IndirectX);
                self.remaining_cycles = 6;
            }
            // SBC Indirect Y
            0xF1 => {
                self.sbc(AddressingMode::IndirectY);
                self.remaining_cycles = 5;
            }
            // DEX
            0xCA => {
                self.dex();
                self.remaining_cycles = 2;
            }
            // DEY
            0x88 => {
                self.dey();
                self.remaining_cycles = 2;
            }
            // INX
            0xE8 => {
                self.inx();
                self.remaining_cycles = 2;
            }
            // INY
            0xC8 => {
                self.iny();
                self.remaining_cycles = 2;
            }
            // ROL A
            0x2A => {
                self.rol(AddressingMode::NoneAddressing);
                self.remaining_cycles = 2;
            }
            // ROL ZP
            0x26 => {
                self.rol(AddressingMode::ZeroPage);
                self.remaining_cycles = 5;
            }
            // ROL ZP X
            0x36 => {
                self.rol(AddressingMode::ZeroPageX);
                self.remaining_cycles = 6;
            }
            // ROL ABS
            0x2E => {
                self.rol(AddressingMode::Absolute);
                self.remaining_cycles = 6;
            }
            // ROL ABS X
            0x3E => {
                self.rol(AddressingMode::AbsoluteX);
                self.remaining_cycles = 7;
            }

            // ROR A
            0x6A => {
                self.ror(AddressingMode::NoneAddressing);
                self.remaining_cycles = 2;
            }
            // ROR ZP
            0x66 => {
                self.ror(AddressingMode::ZeroPage);
                self.remaining_cycles = 5;
            }
            // ROR ZP X
            0x76 => {
                self.ror(AddressingMode::ZeroPageX);
                self.remaining_cycles = 6;
            }
            // ROR ABS
            0x6E => {
                self.ror(AddressingMode::Absolute);
                self.remaining_cycles = 6;
            }
            // ROR ABS X
            0x7E => {
                self.ror(AddressingMode::AbsoluteX);
                self.remaining_cycles = 7;
            }
            // RTI
            0x40 => {
                self.rti();
                self.remaining_cycles = 6;
            }
            // Stub CLD implementation
            0xD8 => {
                self.remaining_cycles = 2;
            }
            // Stub SED implementation
            0xF8 => {
                self.remaining_cycles = 2;
            }
            _ => {
                eprintln!("{:#02x} opcode is not implemented or illegal!", instruction)
            }
        }
    }

    fn ror(&mut self, mode: AddressingMode) {
        if let AddressingMode::NoneAddressing = mode {
            let mut value = self.registers.a;
            self.registers.carry = (value & 1) > 0;
            value >>= 1;
            value |= if self.registers.carry { 0x80 } else { 0 };
            self.registers.a = value;
        } else {
            let addr = self.get_operand_address(mode);
            let mut value = self.read(addr);
            self.registers.carry = (value & 1) > 0;
            value >>= 1;
            value |= if self.registers.carry { 0x80 } else { 0 };
            self.write(addr, value);
        };
    }
    fn rol(&mut self, mode: AddressingMode) {
        if let AddressingMode::NoneAddressing = mode {
            let mut value = self.registers.a;
            self.registers.carry = value >= 0x80;
            value <<= 1;
            value |= if self.registers.carry { 1 } else { 0 };
            self.registers.a = value;
        } else {
            let addr = self.get_operand_address(mode);
            let mut value = self.read(addr);
            self.registers.carry = value >= 0x80;
            value <<= 1;
            value |= if self.registers.carry { 1 } else { 0 };
            self.write(addr, value);
        };
    }
    fn dex(&mut self) {
        self.registers.x = self.registers.x.wrapping_sub(1);
        let x = self.registers.x;
        self.registers.zero = x == 0;
        self.registers.negative = x >= 0x80;
    }
    fn dey(&mut self) {
        self.registers.y = self.registers.y.wrapping_sub(1);
        let y = self.registers.y;
        self.registers.zero = y == 0;
        self.registers.negative = y >= 0x80;
    }
    fn inx(&mut self) {
        self.registers.x = self.registers.x.wrapping_add(1);
        let x = self.registers.x;
        self.registers.zero = x == 0;
        self.registers.negative = x >= 0x80;
    }
    fn iny(&mut self) {
        self.registers.y = self.registers.y.wrapping_add(1);
        let y = self.registers.y;
        self.registers.zero = y == 0;
        self.registers.negative = y >= 0x80;
    }
    fn jmp(&mut self, mode: AddressingMode) {
        let addr = self.get_operand_address(mode);
        let new_pc = self.read_u16(addr);
        self.registers.program_counter = new_pc;
    }
    fn dec(&mut self, mode: AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.read(addr).wrapping_sub(1);
        self.write(addr, value);
        self.registers.zero = value == 0;
        self.registers.negative = value >= 0x80;
    }
    fn inc(&mut self, mode: AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.read(addr).wrapping_add(1);
        self.write(addr, value);
        self.registers.zero = value == 0;
        self.registers.negative = value >= 0x80;
    }
    fn cmp(&mut self, mode: AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.read(addr);
        self.registers.zero = value == self.registers.a;
        self.registers.negative = self.registers.a >= 0x80;
    }
    fn cpy(&mut self, mode: AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.read(addr);
        self.registers.zero = value == self.registers.y;
        self.registers.negative = self.registers.y >= 0x80;
    }
    fn cpx(&mut self, mode: AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.read(addr);
        self.registers.zero = value == self.registers.x;
        self.registers.negative = self.registers.x >= 0x80;
    }
    fn sta(&mut self, mode: AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.read(addr);
        self.registers.a = value;
    }
    fn stx(&mut self, mode: AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.read(addr);
        self.registers.x = value;
    }
    fn sty(&mut self, mode: AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.read(addr);
        self.registers.y = value;
    }
    fn bit(&mut self, mode: AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.read(addr);
        let result = self.registers.a & value;
        self.registers.zero = result == 0;
        self.registers.negative = (value & 0b10000000u8) != 0;
        self.registers.overflow = (value & 0b01000000u8) != 0;
    }
    fn irq(&mut self) {
        if self.registers.interrupt_disable {
            return;
        }
        self.push(((self.registers.program_counter & 0xFF00) >> 8) as u8);
        self.push((self.registers.program_counter) as u8);
        self.push(self.registers.get_flags());
        let new_pc = self.read_u16(0xFFFE);
        self.registers.program_counter = new_pc;
    }
    fn nmi(&mut self) {
        self.push(((self.registers.program_counter & 0xFF00) >> 8) as u8);
        self.push((self.registers.program_counter) as u8);
        self.push(self.registers.get_flags());
        self.registers.interrupt_disable = true;
        let new_pc = self.read_u16(0xFFFA);
        self.registers.program_counter = new_pc;
    }
    fn rti(&mut self) {
        let ps = self.pop();
        self.registers.set_flags_from_byte(ps);
        self.rts();
    }
    fn tax(&mut self) {
        self.registers.x = self.registers.a;
    }
    fn txa(&mut self) {
        self.registers.a = self.registers.x;
    }
    fn tay(&mut self) {
        self.registers.y = self.registers.a;
    }
    fn tya(&mut self) {
        self.registers.a = self.registers.y;
    }
    // X -> sp
    fn txs(&mut self) {
        self.registers.stack_pointer = self.registers.x;
    }
    // sp -> X
    fn tsx(&mut self) {
        self.registers.x = self.registers.stack_pointer;
    }
    fn php(&mut self) {
        self.push(self.registers.get_flags());
    }
    fn plp(&mut self) {
        let ps = self.pop();
        self.registers.set_flags_from_byte(ps);
    }
    fn pha(&mut self) {
        self.push(self.registers.a);
    }
    fn pla(&mut self) {
        self.registers.a = self.pop();
        self.registers.negative = self.registers.a >= 0x80;
        self.registers.zero = self.registers.a == 0;
    }
    fn clc(&mut self) {
        self.registers.carry = false;
    }
    fn sec(&mut self) {
        self.registers.carry = true;
    }
    fn clv(&mut self) {
        self.registers.overflow = false;
    }
    fn jsr(&mut self) {
        self.push(((self.registers.program_counter & 0xFF00) >> 8) as u8);
        self.push((self.registers.program_counter + 2) as u8);
        self.registers.program_counter = self.get_operand_address(AddressingMode::Absolute);
    }
    fn rts(&mut self) {
        let lo = self.pop() as u16;
        let hi = self.pop() as u16;
        let addr = (hi << 8) | lo;
        self.registers.program_counter = addr;
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
        let (result, carry) = self
            .registers
            .a
            .overflowing_add(value + if self.registers.carry { 1 } else { 0 });
        let a = self.registers.a & 0x80 > 0;
        let value_sign = value & 0x80 > 0;
        let result_sign = result & 0x80 > 0;
        // Thanks to OLC for this line
        self.registers.overflow = (a ^ result_sign) && (!(a ^ value_sign));
        self.registers.a = result;
        self.registers.carry = carry;
        self.registers.zero = result == 0;
        self.registers.negative = result >= 0x80;
    }
    fn sbc(&mut self, mode: AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = !self.read(addr);
        let (result, carry) = self
            .registers
            .a
            .overflowing_add(value + if self.registers.carry { 1 } else { 0 });
        let a = self.registers.a & 0x80 > 0;
        let value_sign = value & 0x80 > 0;
        let result_sign = result & 0x80 > 0;
        // Thanks to OLC for this line
        self.registers.overflow = (a ^ result_sign) && (!(a ^ value_sign));
        self.registers.a = result;
        self.registers.carry = carry;
        self.registers.zero = result == 0;
        self.registers.negative = result >= 0x80;
    }
    fn sei(&mut self) {
        self.registers.interrupt_disable = true;
    }
    fn cli(&mut self) {
        self.registers.interrupt_disable = false;
    }
    fn ora(&mut self, mode: AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.read(addr);
        self.registers.a |= value;
        self.registers.zero = self.registers.a == 0;
        self.registers.negative = self.registers.a >= 0x80;
    }

    fn xor(&mut self, mode: AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.read(addr);
        self.registers.a ^= value;
        self.registers.zero = self.registers.a == 0;
        self.registers.negative = self.registers.a >= 0x80;
    }
    fn and(&mut self, mode: AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.read(addr);
        self.registers.a &= value;
        self.registers.zero = self.registers.a == 0;
        self.registers.negative = self.registers.a >= 0x80;
    }

    fn lsr(&mut self, mode: AddressingMode) {
        match mode {
            AddressingMode::NoneAddressing => {
                let value = self.registers.a;

                self.registers.carry = (value & 1) == 1;
                self.registers.negative = value >= 0x80;
                self.registers.zero = value == 1;
                self.registers.a >>= 1;
            }
            _ => {
                let addr = self.get_operand_address(mode);
                let value = self.read(addr);
                self.write(addr, value >> 1);
                self.registers.carry = (value & 1) == 1;
                self.registers.negative = value >= 0x80;
                self.registers.zero = value == 1;
            }
        }
    }

    fn asl(&mut self, mode: AddressingMode) {
        match mode {
            AddressingMode::NoneAddressing => {
                self.registers.carry = self.registers.a >= 0x80;
                self.registers.a <<= 1;
            }
            _ => {
                let addr = self.get_operand_address(mode);
                let value = self.read(addr);
                self.write(addr, value << 1);
                self.registers.carry = value >= 0x80;
            }
        }
    }
    fn bcc(&mut self) {
        if !self.registers.carry {
            let value = self.read(self.registers.program_counter);
            if value > 0x7F {
                self.registers.program_counter -= (!value + 1) as u16;
            } else {
                self.registers.program_counter += value as u16;
            };
        }
        self.registers.program_counter = self.registers.program_counter.wrapping_add(1);
    }

    fn bcs(&mut self) {
        if self.registers.carry {
            let value = self.read(self.registers.program_counter);
            
            if value > 0x7F {
                self.registers.program_counter -= (!value + 1) as u16;
            } else {
                self.registers.program_counter += value as u16;
            }
        }
        self.registers.program_counter = self.registers.program_counter.wrapping_add(1);
    }
    fn bvc(&mut self) {
        if !self.registers.overflow {
            let value = self.read(self.registers.program_counter);
            
            let old_pc = self.registers.program_counter - 1;
            if value > 0x7F {
                self.registers.program_counter -= (!value + 1) as u16;
            } else {
                self.registers.program_counter += value as u16;
            }
        }
        self.registers.program_counter = self.registers.program_counter.wrapping_add(1);
    }
    fn bvs(&mut self) {
        if self.registers.overflow {
            let value = self.read(self.registers.program_counter);
            
            let old_pc = self.registers.program_counter - 1;
            if value > 0x7F {
                self.registers.program_counter -= (!value + 1) as u16;
            } else {
                self.registers.program_counter += value as u16;
            }
        }
        self.registers.program_counter = self.registers.program_counter.wrapping_add(1);
    }

    fn beq(&mut self) {
        if self.registers.zero {
            let value = self.read(self.registers.program_counter);
            
            let old_pc = self.registers.program_counter - 1;
            if value > 0x7F {
                self.registers.program_counter -= (!value + 1) as u16;
            } else {
                self.registers.program_counter += value as u16;
            }
        }
        self.registers.program_counter = self.registers.program_counter.wrapping_add(1);
    }

    fn bne(&mut self) {
        if !self.registers.zero {
            let value = self.read(self.registers.program_counter);

            let old_pc = self.registers.program_counter - 1;
            if value > 0x7F {
                self.registers.program_counter -= (!value + 1) as u16;
            } else {
                self.registers.program_counter += value as u16;
            }
        }
        self.registers.program_counter = self.registers.program_counter.wrapping_add(1);
    }
    fn bmi(&mut self) {
        if self.registers.negative {
            let value = self.read(self.registers.program_counter);

            let old_pc = self.registers.program_counter - 1;
            if value > 0x7F {
                self.registers.program_counter -= (!value + 1) as u16;
            } else {
                self.registers.program_counter += value as u16;
            }
        }
        self.registers.program_counter = self.registers.program_counter.wrapping_add(1);
    }
    fn bpl(&mut self) {
        if !self.registers.negative {
            let value = self.read(self.registers.program_counter);
            // Jump to relative address, it can be negative
            if value > 0x7F {
                self.registers.program_counter -= (!value + 1) as u16;
            } else {
                self.registers.program_counter += value as u16;
            }
        }
        self.registers.program_counter = self.registers.program_counter.wrapping_add(1);
    }

    /**
       Pushes a value onto the stack
    */
    pub fn push(&mut self, value: u8) {
        self.registers.stack_pointer = self.registers.stack_pointer.wrapping_sub(1);
        self.write(self.registers.stack_pointer as u16 + 0x100, value);
    }

    /**
       Pops the value on top of the stack
    */
    pub fn pop(&mut self) -> u8 {
        let old_sp = self.registers.stack_pointer;
        self.registers.stack_pointer = old_sp.wrapping_add(1);
        let value = self.read(old_sp as u16 + 0x100);
        self.write(old_sp as u16 + 0x100, 0);
        value
    }
    pub fn init(&mut self) {
        let low_byte = self.read(0xFFFC);
        let high_byte = self.read(0xFFFD);
        let pc: u16 = (low_byte as u16) | ((high_byte as u16) << 8);
        self.registers.program_counter = pc;
    }
}
