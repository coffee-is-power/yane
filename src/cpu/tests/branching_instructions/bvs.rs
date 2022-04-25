use std::rc::Rc;

use crate::{CPU, cartridge::Cartridge, memory::Memory};

#[test]
fn bvs_jumps_when_overflow_is_true() {
    let mut rom = [0u8; 0x7fff];
    rom[0x3FFC] = 0x00;
    rom[0x3FFD] = 0x80;
    rom[0] = 0x70;
    rom[1] = 4;


    let cartridge = Cartridge::from_rom(rom.to_vec());
    let memory = Memory::new(Rc::new(cartridge));
    let mut cpu = CPU::new(Rc::new(memory));
    cpu.init();
    cpu.registers.overflow = true;
    cpu.exec();
    assert_eq!(cpu.registers.program_counter, 0x8005);
}

#[test]
fn bvs_does_not_jump_when_overflow_is_false() {
    let mut rom = [0u8; 0x7fff];
    rom[0x3FFC] = 0x00;
    rom[0x3FFD] = 0x80;
    rom[0] = 0x70;
    rom[1] = 4;


    let cartridge = Cartridge::from_rom(rom.to_vec());
    let memory = Memory::new(Rc::new(cartridge));
    let mut cpu = CPU::new(Rc::new(memory));
    cpu.init();
    cpu.registers.overflow = false;
    cpu.exec();
    assert_eq!(cpu.registers.program_counter, 0x8001);
}
