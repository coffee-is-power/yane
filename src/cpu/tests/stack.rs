use crate::cpu::registers::Registers;
use crate::CPU;

#[test]
fn push() {
    let mut cpu = CPU::new();
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
    let mut cpu = CPU::new();
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

    rom[0xFFFC - 0x8000] = 0x00;
    rom[0xFFFD - 0x8000] = 0x80;
    rom[0] = 0x48;
    let mut cpu = CPU::with_rom(rom);
    cpu.init();
    cpu.registers.a = 10;
    cpu.exec();
    assert_eq!(cpu.pop(), 10);
}

#[test]
fn pla() {
    let mut rom = [0u8; 0x7fff];

    rom[0xFFFC - 0x8000] = 0x00;
    rom[0xFFFD - 0x8000] = 0x80;
    rom[0] = 0x68;
    let mut cpu = CPU::with_rom(rom);
    cpu.init();
    cpu.push(10);
    cpu.exec();
    assert_eq!(cpu.registers.a, 10);
    assert_eq!(cpu.registers.stack_pointer, 0x24);
}

#[test]
fn php() {
    let mut rom = [0u8; 0x7fff];

    rom[0xFFFC - 0x8000] = 0x00;
    rom[0xFFFD - 0x8000] = 0x80;
    rom[0] = 0x08;
    let mut cpu = CPU::with_rom(rom);
    cpu.init();
    cpu.registers.carry = true;
    cpu.exec();
    assert_eq!(cpu.pop(), 0b10100000);
}

#[test]
fn plp() {
    let mut rom = [0u8; 0x7fff];

    rom[0xFFFC - 0x8000] = 0x00;
    rom[0xFFFD - 0x8000] = 0x80;
    rom[0] = 0x28;
    let mut cpu = CPU::with_rom(rom);
    cpu.init();
    cpu.push(0b10100000);
    cpu.exec();
    assert!(cpu.registers.carry);
    assert!(cpu.registers.interrupt_disable);
    assert!(!cpu.registers.overflow);
    assert!(!cpu.registers.negative);
    assert!(!cpu.registers.zero);
}
