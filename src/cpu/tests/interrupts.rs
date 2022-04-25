use std::rc::Rc;

use crate::{CPU, cartridge::Cartridge, memory::Memory};

#[test]
fn cli_enables_interrupts() {
    let mut rom = [0u8; 0x7fff];

    rom[0x3FFC] = 0x00;
    rom[0x3FFD] = 0x80;
    rom[0] = 0x58;


    let cartridge = Cartridge::from_rom(rom.to_vec());
    let memory = Memory::new(Rc::new(cartridge));
    let mut cpu = CPU::new(Rc::new(memory));
    cpu.init();
    cpu.registers.interrupt_disable = true;
    cpu.exec();
    assert_eq!(cpu.registers.interrupt_disable, false);
}

#[test]
fn sei_disables_interrupts() {
    let mut rom = [0u8; 0x7fff];

    rom[0x3FFC] = 0x00;
    rom[0x3FFD] = 0x80;
    rom[0] = 0x78;


    let cartridge = Cartridge::from_rom(rom.to_vec());
    let memory = Memory::new(Rc::new(cartridge));
    let mut cpu = CPU::new(Rc::new(memory));
    cpu.init();
    cpu.registers.interrupt_disable = false;
    cpu.exec();
    assert_eq!(cpu.registers.interrupt_disable, true);
}
