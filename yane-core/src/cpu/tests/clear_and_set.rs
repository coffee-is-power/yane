use std::{rc::Rc, cell::RefCell};


use crate::{CPU, cartridge::Cartridge, memory::Memory, ppu::PPU};
#[test]
fn clc() {
    let mut rom = [0u8; 0x7fff];

    rom[0x3FFC] = 0x00;
    rom[0x3FFD] = 0x80;
    rom[0] = 0x18;


    let cartridge = Rc::new(RefCell::new(Cartridge::from_rom(rom.to_vec())));
    let ppu = Rc::new(RefCell::new(PPU::new(&cartridge)));
    let memory = Memory::new(&cartridge, &ppu);
    let mut cpu = CPU::new(memory);
    cpu.init();
    cpu.registers.carry = true;
    cpu.exec();
    assert!(!cpu.registers.carry);
}
#[test]
fn sec() {
    let mut rom = [0u8; 0x7fff];

    rom[0x3FFC] = 0x00;
    rom[0x3FFD] = 0x80;
    rom[0] = 0x38;


    let cartridge = Rc::new(RefCell::new(Cartridge::from_rom(rom.to_vec())));
    let ppu = Rc::new(RefCell::new(PPU::new(&cartridge)));
    let memory = Memory::new(&cartridge, &ppu);
    let mut cpu = CPU::new(memory);
    cpu.init();
    cpu.registers.carry = false;
    cpu.exec();
    assert!(cpu.registers.carry);
}
#[test]
fn clv() {
    let mut rom = [0u8; 0x7fff];

    rom[0x3FFC] = 0x00;
    rom[0x3FFD] = 0x80;
    rom[0] = 0xB8;


    let cartridge = Rc::new(RefCell::new(Cartridge::from_rom(rom.to_vec())));
    let ppu = Rc::new(RefCell::new(PPU::new(&cartridge)));
    let memory = Memory::new(&cartridge, &ppu);
    let mut cpu = CPU::new(memory);
    cpu.init();
    cpu.registers.overflow = true;
    cpu.exec();
    assert!(!cpu.registers.overflow);
}
