use crate::CPU;

#[test]
fn accumulator() {
    let mut rom = [0u8; 0x7fff];

    rom[0x3FFC] = 0x00;
    rom[0x3FFD] = 0x80;
    rom[0] = 0x4A;

    let mut cpu = CPU::from_rom(&rom);
    cpu.registers.a = 3;
    cpu.init();
    cpu.exec();
    assert_eq!(cpu.registers.a, 3 >> 1);
    assert_eq!(cpu.registers.negative, false);
    assert_eq!(cpu.registers.zero, false);
    assert_eq!(cpu.registers.carry, true);
}

#[test]
fn zeropage() {
    let mut rom = [0u8; 0x7fff];

    rom[0x3FFC] = 0x00;
    rom[0x3FFD] = 0x80;
    rom[0] = 0x46;
    rom[1] = 0;


    let mut cpu = CPU::from_rom(&rom);
    cpu.memory.borrow_mut().ram[0] = 0x3;
    cpu.init();
    cpu.exec();
    assert_eq!(cpu.memory.borrow_mut().ram[0], 3 >> 1);
    assert_eq!(cpu.registers.negative, false);
    assert_eq!(cpu.registers.zero, false);
    assert_eq!(cpu.registers.carry, true);
}

#[test]
fn zeropage_x() {
    let mut rom = [0u8; 0x7fff];

    rom[0x3FFC] = 0x00;
    rom[0x3FFD] = 0x80;
    rom[0] = 0x56;
    rom[1] = 0;


    let mut cpu = CPU::from_rom(&rom);
    cpu.memory.borrow_mut().ram[1] = 0x3;
    cpu.registers.x = 1;
    cpu.init();
    cpu.exec();
    assert_eq!(cpu.memory.borrow_mut().ram[1], 3 >> 1);
    assert_eq!(cpu.registers.negative, false);
    assert_eq!(cpu.registers.zero, false);
    assert_eq!(cpu.registers.carry, true);
}

#[test]
fn abs() {
    let mut rom = [0u8; 0x7fff];

    rom[0x3FFC] = 0x00;
    rom[0x3FFD] = 0x80;
    rom[0] = 0x4E;
    rom[1] = 0x11;
    rom[2] = 0x5;


    let mut cpu = CPU::from_rom(&rom);
    cpu.memory.borrow_mut().ram[0x511] = 0x3;
    cpu.init();
    cpu.exec();
    assert_eq!(cpu.memory.borrow_mut().ram[0x511], 3 >> 1);
    assert_eq!(cpu.registers.negative, false);
    assert_eq!(cpu.registers.zero, false);
    assert_eq!(cpu.registers.carry, true);
}

#[test]
fn abs_x() {
    let mut rom = [0u8; 0x7fff];

    rom[0x3FFC] = 0x00;
    rom[0x3FFD] = 0x80;
    rom[0] = 0x5E;
    rom[1] = 0x11;
    rom[2] = 0x05;

    let mut cpu = CPU::from_rom(&rom);
    cpu.memory.borrow_mut().ram[0x512] = 0x3;
    cpu.registers.x = 1;
    cpu.init();
    cpu.exec();
    assert_eq!(cpu.memory.borrow_mut().ram[0x512], 3 >> 1);
    assert_eq!(cpu.registers.negative, false);
    assert_eq!(cpu.registers.zero, false);
    assert_eq!(cpu.registers.carry, true);
}
