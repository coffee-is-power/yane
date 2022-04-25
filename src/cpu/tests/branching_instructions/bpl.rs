use std::rc::Rc;

use crate::{CPU, cartridge::Cartridge, memory::Memory};

#[test]
fn bpl_jumps_when_negative_is_false() {
    let mut rom = [0u8; 0x7fff];
    rom[0x3FFC] = 0x00;
    rom[0x3FFD] = 0x80;
    rom[0] = 0x10;
    rom[1] = 3;


    let cartridge = Cartridge::from_rom(rom.to_vec());
    let memory = Memory::new(Rc::new(cartridge));
    let mut cpu = CPU::new(Rc::new(memory));
    cpu.init();
    cpu.registers.negative = false;
    cpu.exec();
    assert_eq!(cpu.registers.program_counter, 0x8005);
}

#[test]
fn bpl_does_not_jump_when_negative_is_true() {
    let mut rom = [0u8; 0x7fff];
    rom[0x3FFC] = 0x00;
    rom[0x3FFD] = 0x80;
    rom[0] = 0x10;
    rom[1] = 3;


    let cartridge = Cartridge::from_rom(rom.to_vec());
    let memory = Memory::new(Rc::new(cartridge));
    let mut cpu = CPU::new(Rc::new(memory));
    cpu.init();
    cpu.registers.negative = true;
    cpu.exec();
    assert_eq!(cpu.registers.program_counter, 0x8001);
}
