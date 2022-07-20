use std::{rc::Rc, sync::{Arc, Mutex}};

use crate::{CPU, cartridge::Cartridge, memory::Memory, ppu::PPU};
#[test]
fn immediate_instruction() {
    let mut rom = [0u8; 0x7fff];

    rom[0x3FFC] = 0x00;
    rom[0x3FFD] = 0x80;
    rom[0] = 0xA9;
    rom[1] = 10;
    rom[2] = 0x29;
    rom[3] = 7;

    let cartridge = Arc::new(Mutex::new(Cartridge::from_rom(rom.to_vec())));
    let memory = Memory::new(cartridge.clone(), Arc::new(Mutex::new(PPU::new(cartridge.clone()))));
    let mut cpu = CPU::new(Rc::new(memory));
    cpu.init();
    cpu.exec();
    cpu.exec();
    assert_eq!(cpu.registers.a, 10 & 7);
}

#[test]
fn zeropage() {
    let mut rom = [0u8; 0x7fff];

    rom[0x3FFC] = 0x00;
    rom[0x3FFD] = 0x80;
    rom[0] = 0xA9;
    rom[1] = 10;
    rom[2] = 0x25;
    rom[3] = 61;


    let cartridge = Arc::new(Mutex::new(Cartridge::from_rom(rom.to_vec())));
    let memory = Memory::new(cartridge.clone(), Arc::new(Mutex::new(PPU::new(cartridge.clone()))));
    let mut cpu = CPU::new(Rc::new(memory));
    cpu.write(61, 7);
    cpu.init();
    cpu.exec();
    cpu.exec();
    assert_eq!(cpu.registers.a, 10 & 7);
}

#[test]
fn zeropage_x() {
    let mut rom = [0u8; 0x7fff];

    rom[0x3FFC] = 0x00;
    rom[0x3FFD] = 0x80;
    rom[0] = 0xA9;
    rom[1] = 10;
    rom[2] = 0x35;
    rom[3] = 61;


    let cartridge = Arc::new(Mutex::new(Cartridge::from_rom(rom.to_vec())));
    let memory = Memory::new(cartridge.clone(), Arc::new(Mutex::new(PPU::new(cartridge.clone()))));
    let mut cpu = CPU::new(Rc::new(memory));
    cpu.registers.x = 1;
    cpu.write(62, 7);
    cpu.init();
    cpu.exec();
    cpu.exec();
    assert_eq!(cpu.registers.a, 10 & 7);
}

#[test]
fn abs() {
    let mut rom = [0u8; 0x7fff];

    rom[0x3FFC] = 0x00;
    rom[0x3FFD] = 0x80;
    rom[0] = 0xA9;
    rom[1] = 10;

    rom[2] = 0x2D;
    rom[3] = 0x10;
    rom[4] = 0x10;


    let cartridge = Arc::new(Mutex::new(Cartridge::from_rom(rom.to_vec())));
    let memory = Memory::new(cartridge.clone(), Arc::new(Mutex::new(PPU::new(cartridge.clone()))));
    let mut cpu = CPU::new(Rc::new(memory));
    cpu.write(0x1010, 7);
    cpu.init();
    cpu.exec();
    cpu.exec();
    assert_eq!(cpu.registers.a, 10 & 7);
}

#[test]
fn abs_x() {
    let mut rom = [0u8; 0x7fff];

    rom[0x3FFC] = 0x00;
    rom[0x3FFD] = 0x80;
    rom[0] = 0xA9;
    rom[1] = 10;

    rom[2] = 0x3D;
    rom[3] = 0x10;
    rom[4] = 0x10;


    let cartridge = Arc::new(Mutex::new(Cartridge::from_rom(rom.to_vec())));
    let memory = Memory::new(cartridge.clone(), Arc::new(Mutex::new(PPU::new(cartridge.clone()))));
    let mut cpu = CPU::new(Rc::new(memory));
    cpu.registers.x = 1;
    cpu.write(0x1011, 7);
    cpu.init();
    cpu.exec();
    cpu.exec();
    assert_eq!(cpu.registers.a, 10 & 7);
}

#[test]
fn abs_y() {
    let mut rom = [0u8; 0x7fff];

    rom[0x3FFC] = 0x00;
    rom[0x3FFD] = 0x80;
    rom[0] = 0xA9;
    rom[1] = 10;

    rom[2] = 0x39;
    rom[3] = 0x10;
    rom[4] = 0x10;


    let cartridge = Arc::new(Mutex::new(Cartridge::from_rom(rom.to_vec())));
    let memory = Memory::new(cartridge.clone(), Arc::new(Mutex::new(PPU::new(cartridge.clone()))));
    let mut cpu = CPU::new(Rc::new(memory));
    cpu.registers.y = 1;
    cpu.write(0x1011, 7);
    cpu.init();
    cpu.exec();
    cpu.exec();
    assert_eq!(cpu.registers.a, 10 & 7);
}

#[test]
fn indirect_y() {
    let mut rom = [0u8; 0x7fff];

    rom[0x3FFC] = 0x00;
    rom[0x3FFD] = 0x80;
    rom[0] = 0xA9;
    rom[1] = 10;

    rom[2] = 0x31;
    rom[3] = 0x11;


    let cartridge = Arc::new(Mutex::new(Cartridge::from_rom(rom.to_vec())));
    let memory = Memory::new(cartridge.clone(), Arc::new(Mutex::new(PPU::new(cartridge.clone()))));
    let mut cpu = CPU::new(Rc::new(memory));
    cpu.registers.y = 1;
    cpu.write(0x11, 0x11);
    cpu.write(0x12, 0x12);
    cpu.write(0x1212, 7);
    cpu.init();
    cpu.exec();
    cpu.exec();
    assert_eq!(cpu.registers.a, 10 & 7);
}

#[test]
fn indirect_x() {
    let mut rom = [0u8; 0x7fff];

    rom[0x3FFC] = 0x00;
    rom[0x3FFD] = 0x80;
    rom[0] = 0xA9;
    rom[1] = 10;

    rom[2] = 0x21;
    rom[3] = 0x11;


    let cartridge = Arc::new(Mutex::new(Cartridge::from_rom(rom.to_vec())));
    let memory = Memory::new(cartridge.clone(), Arc::new(Mutex::new(PPU::new(cartridge.clone()))));
    let mut cpu = CPU::new(Rc::new(memory));
    cpu.registers.x = 1;
    cpu.write(0x12, 0x11);
    cpu.write(0x13, 0x12);
    cpu.write(0x1211, 7);
    cpu.init();
    cpu.exec();
    cpu.exec();
    assert_eq!(cpu.registers.a, 10 & 7);
}
