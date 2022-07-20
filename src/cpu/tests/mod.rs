mod bitwise_instructions;
mod branching_instructions;
mod clear_and_set;
mod interrupts;
mod load_instructions;
mod math_operations;
mod stack;
mod subroutines;
mod transfer_instructions;

use std::{rc::Rc, sync::{Arc, Mutex}};

use crate::{CPU, cartridge::Cartridge, memory::Memory, ppu::PPU};

#[test]
fn init_cpu_sets_correct_pc() {
    let mut rom = [0u8; 0x7fff];

    rom[0x3FFC] = 0x00;
    rom[0x3FFD] = 0x80;


    let cartridge = Arc::new(Mutex::new(Cartridge::from_rom(rom.to_vec())));
    let memory = Memory::new(cartridge.clone(), Arc::new(Mutex::new(PPU::new(cartridge.clone()))));
    let mut cpu = CPU::new(Rc::new(memory));
    cpu.init();
    assert_eq!(
        cpu.registers.program_counter, 0x8000,
        "The PC register must be set to the address in 0xFFFC and 0xFFFD"
    );
}
#[test]
fn ram_write_test() {
    let cartridge = Arc::new(Mutex::new(Cartridge::from_rom(vec![])));
    let memory = Memory::new(cartridge.clone(), Arc::new(Mutex::new(PPU::new(cartridge.clone()))));
    let mut cpu = CPU::new(Rc::new(memory));
    cpu.write(4, 10);
    assert_eq!(
        Rc::get_mut(&mut cpu.memory).unwrap().ram[4], 10,
        "The ram must hold the values written to it"
    );
}

#[test]
fn ram_read_test() {
    let cartridge = Arc::new(Mutex::new(Cartridge::from_rom(vec![])));
    let memory = Memory::new(cartridge.clone(), Arc::new(Mutex::new(PPU::new(cartridge.clone()))));
    let mut cpu = CPU::new(Rc::new(memory));
    Rc::get_mut(&mut cpu.memory).unwrap().ram[4] = 10;
    assert_eq!(
        cpu.read(4),
        10,
        "The read method should return 10 from the ram after writing to it"
    );
}
#[test]
fn read_u16_test() {
    let mut rom = [0u8; 0x7fff];

    rom[(0xFFFC - 0x8000) & 0x3fff] = 0x00;
    rom[(0xFFFD - 0x8000) & 0x3fff] = 0x80;
    let cartridge = Arc::new(Mutex::new(Cartridge::from_rom(rom.to_vec())));
    let memory = Memory::new(cartridge.clone(), Arc::new(Mutex::new(PPU::new(cartridge.clone()))));
    let mut cpu = CPU::new(Rc::new(memory));
    assert_eq!(cpu.read_u16(0xFFFC), 0x8000)
}
