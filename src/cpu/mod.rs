pub mod registers;
use crate::memory::Memory;
use registers::Registers;
const CPU_CLOCK_DELAY: f64 = 5.58730074e-7;
fn sleep_cycles(cycles: u32){
    std::thread::sleep(
        std::time::Duration::from_secs_f64(
            CPU_CLOCK_DELAY * cycles as f64
        )
    );
}

#[derive(Debug)]
pub enum AddressingMode {
   Immediate,
   ZeroPage,
   ZeroPage_X,
   ZeroPage_Y,
   Absolute,
   Absolute_X,
   Absolute_Y,
   Indirect_X,
   Indirect_Y,
   NoneAddressing,
}

#[derive(Debug)]
pub struct CPU {
    pub registers: Registers,
    pub memory: Memory
}
impl CPU {
    pub fn new() -> Self {
        Self {
            registers: Registers::new(),
            memory: Memory::new()
        }
    }
    fn read_u16(&self, addr: u16) -> u16 {
        self.read(addr) as u16 | ((self.read(addr+1) as u16) << 8)
    }
    fn get_operand_address(&self, mode: &AddressingMode) -> u16 {

        match mode {
            AddressingMode::Immediate => self.registers.program_counter,
 
            AddressingMode::ZeroPage  => self.read(self.registers.program_counter) as u16,
           
            AddressingMode::Absolute => self.read_u16(self.registers.program_counter),
         
            AddressingMode::ZeroPage_X => {
                let pos = self.read(self.registers.program_counter);
                let addr = pos.wrapping_add(self.registers.x) as u16;
                addr
            }
            AddressingMode::ZeroPage_Y => {
                let pos = self.read(self.registers.program_counter);
                let addr = pos.wrapping_add(self.registers.y) as u16;
                addr
            }
 
            AddressingMode::Absolute_X => {
                let base = self.read_u16(self.registers.program_counter);
                let addr = base.wrapping_add(self.registers.x as u16);
                addr
            }
            AddressingMode::Absolute_Y => {
                let base = self.read_u16(self.registers.program_counter);
                let addr = base.wrapping_add(self.registers.y as u16);
                addr
            }
 
            AddressingMode::Indirect_X => {
                let base = self.read(self.registers.program_counter);
 
                let ptr: u8 = (base as u8).wrapping_add(self.registers.x);
                let lo = self.read(ptr as u16);
                let hi = self.read(ptr.wrapping_add(1) as u16);
                (hi as u16) << 8 | (lo as u16)
            }
            AddressingMode::Indirect_Y => {
                let base = self.read(self.registers.program_counter);
 
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
    pub fn write(&mut self, address: u16, data: u8){
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
        match instruction {
            0xea /* NOP */=> {
                self.registers.program_counter += 1;
                sleep_cycles(2);
            },
            0x09 /*ORA immediate value*/ => {
                self.registers.a |= self.read(self.registers.program_counter+1);
                self.registers.program_counter += 2;
                sleep_cycles(2);
            },
            0x05 /*ORA zeropage addr*/ => {
                self.registers.a |= self.read(self.read(self.registers.program_counter+1) as u16);
                self.registers.program_counter += 2;
                sleep_cycles(3);
            },
            0x15 /*ORA zeropage addr x-indexed*/ => {
                let addr = self.read(self.registers.program_counter+1) as u16;
                self.registers.a |= self.read((addr+self.registers.x as u16) & 0xFF);
                self.registers.program_counter += 2;
                sleep_cycles(4);
            },

            0x0D /*ORA abs addr*/ => {
                let low_byte = self.read(self.registers.program_counter+1) as u16;
                let high_byte = self.read(self.registers.program_counter+2) as u16;
                let addr = low_byte | (high_byte << 8);
                self.registers.a |= self.read(addr);
                self.registers.program_counter += 3;
                sleep_cycles(4);
            },
            0x1D /*ORA abs addr + x*/ => {
                let low_byte = self.read(self.registers.program_counter+1) as u16;
                let high_byte = self.read(self.registers.program_counter+2) as u16;
                let addr = (low_byte | (high_byte << 8)) + self.registers.x as u16;
                
                self.registers.a |= self.read(addr);
                self.registers.program_counter += 3;
                sleep_cycles(4);
            },
            _ => todo!("{:#02x} opcode is not implemented yet!", instruction)
        }
    }
    fn lda(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.read(addr);
        self.registers.a = value;
        self.update_zero_and_negative_flags(self.registers.a);
    }
    pub fn init(&mut self){
        let low_byte = self.read(0xFFFC);
        let high_byte = self.read(0xFFFD);
        let pc: u16 = (low_byte as u16) | ((high_byte as u16) << 8);
        self.registers.program_counter = pc;
    }
}