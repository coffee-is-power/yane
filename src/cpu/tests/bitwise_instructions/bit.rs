use std::rc::Rc;

use crate::{CPU, cartridge::Cartridge, memory::Memory, ppu::PPU};

#[test]
fn bit_zp() {
    let mut rom = [0u8; 0x7fff];

    rom[0x3FFC] = 0x00;
    rom[0x3FFD] = 0x80;
    rom[0] = 0x24;
    rom[1] = 0x10;

    
    let cartridge = Rc::new(Cartridge::from_rom(rom.to_vec()));
    let memory = Memory::new(cartridge.clone(), Rc::new(PPU::new(cartridge.clone())));
    let mut cpu = CPU::new(Rc::new(memory));
    cpu.registers.a = 0xF;
    cpu.write(0x0010, 0b11000000);
    cpu.init();
    cpu.exec();
    assert!(cpu.registers.zero);
    assert!(cpu.registers.negative);
    assert!(cpu.registers.overflow);
}

#[test]
fn bit_abs() {
    let mut rom = [0u8; 0x7fff];

    rom[0x3FFC] = 0x00;
    rom[0x3FFD] = 0x80;

    rom[0] = 0x2C;
    rom[1] = 0x10;
    rom[2] = 0x10;


    let cartridge = Rc::new(Cartridge::from_rom(rom.to_vec()));
    let memory = Memory::new(cartridge.clone(), Rc::new(PPU::new(cartridge.clone())));
    let mut cpu = CPU::new(Rc::new(memory));
    cpu.registers.a = 0xF;
    cpu.write(0x1010, 0b11000000);
    cpu.init();
    cpu.exec();
    assert!(cpu.registers.zero);
    assert!(cpu.registers.negative);
    assert!(cpu.registers.overflow);
}
