use std::{rc::Rc, sync::{Arc, Mutex}};

use crate::{CPU, cartridge::Cartridge, memory::Memory, ppu::PPU};

#[test]
fn push() {
    
    let cartridge = Arc::new(Mutex::new(Cartridge::from_rom(vec![])));
    let memory = Memory::new(cartridge.clone(), Arc::new(Mutex::new(PPU::new(cartridge.clone()))));
    let mut cpu = CPU::new(Rc::new(memory));
    cpu.push(10);
    assert_eq!(
        cpu.registers.stack_pointer, 0x23,
        "The sp must be decremented"
    );
    assert_eq!(
        cpu.read(0x123),
        10,
        "The pushed value should be written to memory"
    );
}

#[test]
fn pop() {
    let cartridge = Arc::new(Mutex::new(Cartridge::from_rom(vec![])));
    let memory = Memory::new(cartridge.clone(), Arc::new(Mutex::new(PPU::new(cartridge.clone()))));
    let mut cpu = CPU::new(Rc::new(memory));
    cpu.push(10);

    let value = cpu.pop();
    assert_eq!(
        cpu.registers.stack_pointer, 0x24,
        "The sp must be incremented"
    );
    assert_eq!(
        cpu.read(0x123),
        0,
        "The popped value should be deleted from memory"
    );
    assert_eq!(value, 10);
}
#[test]
fn pha() {
    let mut rom = [0u8; 0x7fff];

    rom[0x3FFC] = 0x00;
    rom[0x3FFD] = 0x80;
    rom[0] = 0x48;


    let cartridge = Arc::new(Mutex::new(Cartridge::from_rom(rom.to_vec())));
    let memory = Memory::new(cartridge.clone(), Arc::new(Mutex::new(PPU::new(cartridge.clone()))));
    let mut cpu = CPU::new(Rc::new(memory));
    cpu.init();
    cpu.registers.a = 10;
    cpu.exec();
    assert_eq!(cpu.pop(), 10);
}

#[test]
fn pla() {
    let mut rom = [0u8; 0x7fff];

    rom[0x3FFC] = 0x00;
    rom[0x3FFD] = 0x80;
    rom[0] = 0x68;


    let cartridge = Arc::new(Mutex::new(Cartridge::from_rom(rom.to_vec())));
    let memory = Memory::new(cartridge.clone(), Arc::new(Mutex::new(PPU::new(cartridge.clone()))));
    let mut cpu = CPU::new(Rc::new(memory));
    cpu.init();
    cpu.push(10);
    cpu.exec();
    assert_eq!(cpu.registers.a, 10);
    assert_eq!(cpu.registers.stack_pointer, 0x24);
}

#[test]
fn php() {
    let mut rom = [0u8; 0x7fff];

    rom[0x3FFC] = 0x00;
    rom[0x3FFD] = 0x80;
    rom[0] = 0x08;


    let cartridge = Arc::new(Mutex::new(Cartridge::from_rom(rom.to_vec())));
    let memory = Memory::new(cartridge.clone(), Arc::new(Mutex::new(PPU::new(cartridge.clone()))));
    let mut cpu = CPU::new(Rc::new(memory));
    cpu.init();
    cpu.registers.carry = true;
    cpu.exec();
    assert_eq!(cpu.pop(), 0b10100000);
}

#[test]
fn plp() {
    let mut rom = [0u8; 0x7fff];

    rom[0x3FFC] = 0x00;
    rom[0x3FFD] = 0x80;
    rom[0] = 0x28;


    let cartridge = Arc::new(Mutex::new(Cartridge::from_rom(rom.to_vec())));
    let memory = Memory::new(cartridge.clone(), Arc::new(Mutex::new(PPU::new(cartridge.clone()))));
    let mut cpu = CPU::new(Rc::new(memory));
    cpu.init();
    cpu.push(0b10100000);
    cpu.exec();
    assert!(cpu.registers.carry);
    assert!(cpu.registers.interrupt_disable);
    assert!(!cpu.registers.overflow);
    assert!(!cpu.registers.negative);
    assert!(!cpu.registers.zero);
}
