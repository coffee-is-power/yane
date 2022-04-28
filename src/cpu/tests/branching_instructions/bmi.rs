use std::rc::Rc;

use crate::{CPU, cartridge::Cartridge, memory::Memory, ppu::PPU};

#[test]
fn bmi_jumps_when_negative_is_true() {
    let mut rom = [0u8; 0x7fff];
    rom[0x3FFC] = 0x00;
    rom[0x3FFD] = 0x80;
    rom[0] = 0x30;
    rom[1] = 4;


    let cartridge = Rc::new(Cartridge::from_rom(rom.to_vec()));
    let memory = Memory::new(cartridge.clone(), Rc::new(PPU::new(cartridge.clone())));
    let mut cpu = CPU::new(Rc::new(memory));
    cpu.init();
    cpu.registers.negative = true;
    cpu.exec();
    assert_eq!(cpu.registers.program_counter, 0x8005);
}

#[test]
fn bmi_does_not_jump_when_negative_is_false() {
    let mut rom = [0u8; 0x7fff];
    rom[0x3FFC] = 0x00;
    rom[0x3FFD] = 0x80;
    rom[0] = 0x30;
    rom[1] = 4;


    let cartridge = Rc::new(Cartridge::from_rom(rom.to_vec()));
    let memory = Memory::new(cartridge.clone(), Rc::new(PPU::new(cartridge.clone())));
    let mut cpu = CPU::new(Rc::new(memory));
    cpu.init();
    cpu.registers.negative = false;
    cpu.exec();
    assert_eq!(cpu.registers.program_counter, 0x8001);
}
