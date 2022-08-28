use crate::CPU;#[test]
fn clc() {
    let mut rom = [0u8; 0x7fff];
    rom[0x3FFC] = 0x00;
    rom[0x3FFD] = 0x80;
    rom[0] = 0x18;
    let mut cpu = CPU::from_rom(&rom);
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


let mut cpu = CPU::from_rom(&rom);
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


let mut cpu = CPU::from_rom(&rom);
    cpu.init();
    cpu.registers.overflow = true;
    cpu.exec();
    assert!(!cpu.registers.overflow);
}
