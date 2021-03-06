use std::{rc::Rc, sync::{Arc, Mutex}};

use crate::{CPU, cartridge::Cartridge, memory::Memory, ppu::PPU};

#[test]
fn jsr() {
    let mut rom = [0u8; 0x7fff];

    rom[0x3FFC] = 0x00;
    rom[0x3FFD] = 0x80;
    rom[0] = 0x20;
    rom[1] = 0x50;
    rom[2] = 0x80;


    let cartridge = Arc::new(Mutex::new(Cartridge::from_rom(rom.to_vec())));
    let memory = Memory::new(cartridge.clone(), Arc::new(Mutex::new(PPU::new(cartridge.clone()))));
    let mut cpu = CPU::new(Rc::new(memory));
    cpu.init();
    cpu.exec();
    assert_eq!(cpu.registers.stack_pointer, 0x22);

    let hi = cpu.read(0x123) as u16;
    let lo = cpu.read(0x122) as u16;
    let addr = (hi << 8) | lo;
    assert_eq!(addr, 0x8003);
    assert_eq!(cpu.registers.program_counter, 0x8050);
}
#[test]
fn rts() {
    let mut rom = [0u8; 0x7fff];

    rom[0x3FFC] = 0x00;
    rom[0x3FFD] = 0x80;
    rom[0] = 0x20;
    rom[1] = 0x50;
    rom[2] = 0x80;
    rom[0x50] = 0x60;


    let cartridge = Arc::new(Mutex::new(Cartridge::from_rom(rom.to_vec())));
    let memory = Memory::new(cartridge.clone(), Arc::new(Mutex::new(PPU::new(cartridge.clone()))));
    let mut cpu = CPU::new(Rc::new(memory));
    cpu.init();
    cpu.exec();
    cpu.exec();
    assert_eq!(cpu.registers.stack_pointer, 0x24);
    assert_eq!(cpu.registers.program_counter, 0x8003);
}
