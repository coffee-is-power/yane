use std::{rc::Rc, sync::{Arc, Mutex}};

use crate::{CPU, cartridge::Cartridge, memory::Memory, ppu::PPU};
#[test]
fn immediate() {
    let mut rom = [0u8; 0x7fff];

    rom[0x3FFC] = 0x00;
    rom[0x3FFD] = 0x80;
    rom[0] = 0xA0;
    rom[1] = 0x10;


    let cartridge = Arc::new(Mutex::new(Cartridge::from_rom(rom.to_vec())));
    let memory = Memory::new(cartridge.clone(), Arc::new(Mutex::new(PPU::new(cartridge.clone()))));
    let mut cpu = CPU::new(Rc::new(memory));
    cpu.init();
    cpu.exec();
    assert_eq!(cpu.registers.y, 0x10);
}

#[test]
fn zero_page() {
    let mut rom = [0u8; 0x7fff];

    rom[0x3FFC] = 0x00;
    rom[0x3FFD] = 0x80;
    rom[0] = 0xA4;
    rom[1] = 0x10;


    let cartridge = Arc::new(Mutex::new(Cartridge::from_rom(rom.to_vec())));
    let memory = Memory::new(cartridge.clone(), Arc::new(Mutex::new(PPU::new(cartridge.clone()))));
    let mut cpu = CPU::new(Rc::new(memory));
    cpu.write(0x10, 0x4);
    cpu.init();
    cpu.exec();
    assert_eq!(cpu.registers.y, 0x4);
}

#[test]
fn zero_page_x() {
    let mut rom = [0u8; 0x7fff];

    rom[0x3FFC] = 0x00;
    rom[0x3FFD] = 0x80;
    rom[0] = 0xB4;
    rom[1] = 0x10;


    let cartridge = Arc::new(Mutex::new(Cartridge::from_rom(rom.to_vec())));
    let memory = Memory::new(cartridge.clone(), Arc::new(Mutex::new(PPU::new(cartridge.clone()))));
    let mut cpu = CPU::new(Rc::new(memory));
    cpu.registers.x = 2;
    cpu.write(0x12, 0x4);
    cpu.init();
    cpu.exec();
    assert_eq!(cpu.registers.y, 0x4);
}

#[test]
fn abs() {
    let mut rom = [0u8; 0x7fff];

    rom[0x3FFC] = 0x00;
    rom[0x3FFD] = 0x80;
    rom[0] = 0xAC;
    rom[1] = 0x00;
    rom[2] = 0x10;


    let cartridge = Arc::new(Mutex::new(Cartridge::from_rom(rom.to_vec())));
    let memory = Memory::new(cartridge.clone(), Arc::new(Mutex::new(PPU::new(cartridge.clone()))));
    let mut cpu = CPU::new(Rc::new(memory));
    cpu.write(0x1000, 0x4);
    cpu.init();
    cpu.exec();
    assert_eq!(cpu.registers.y, 0x4);
}

#[test]
fn abs_x() {
    let mut rom = [0u8; 0x7fff];

    rom[0x3FFC] = 0x00;
    rom[0x3FFD] = 0x80;
    rom[0] = 0xBC;
    rom[1] = 0x00;
    rom[2] = 0x10;


    let cartridge = Arc::new(Mutex::new(Cartridge::from_rom(rom.to_vec())));
    let memory = Memory::new(cartridge.clone(), Arc::new(Mutex::new(PPU::new(cartridge.clone()))));
    let mut cpu = CPU::new(Rc::new(memory));
    cpu.registers.x = 2;
    cpu.write(0x1002, 0x4);
    cpu.init();
    cpu.exec();
    assert_eq!(cpu.registers.y, 0x4);
}
