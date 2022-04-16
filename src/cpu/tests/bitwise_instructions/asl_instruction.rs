use crate::CPU;

#[test]
fn accumulator() {
    let mut rom = [0u8; 0x7fff];

    rom[0xFFFC - 0x8000] = 0x00;
    rom[0xFFFD - 0x8000] = 0x80;
    rom[0] = 0x0A;
    let mut cpu = CPU::with_rom(rom);
    cpu.registers.a = 3;
    cpu.init();
    cpu.exec();
    assert_eq!(cpu.registers.a, 3 << 1);
    assert_eq!(cpu.registers.negative, false);
    assert_eq!(cpu.registers.zero, false);
    assert_eq!(cpu.registers.carry, false);
}

#[test]
fn zeropage() {
    let mut rom = [0u8; 0x7fff];

    rom[0xFFFC - 0x8000] = 0x00;
    rom[0xFFFD - 0x8000] = 0x80;
    rom[0] = 0x06;
    rom[1] = 0;
    let mut cpu = CPU::with_rom(rom);
    cpu.memory.ram[0] = 0x3;
    cpu.init();
    cpu.exec();
    assert_eq!(cpu.memory.ram[0], 3 << 1);
    assert_eq!(cpu.registers.negative, false);
    assert_eq!(cpu.registers.zero, false);
    assert_eq!(cpu.registers.carry, false);
}

#[test]
fn zeropage_x() {
    let mut rom = [0u8; 0x7fff];

    rom[0xFFFC - 0x8000] = 0x00;
    rom[0xFFFD - 0x8000] = 0x80;
    rom[0] = 0x16;
    rom[1] = 0;
    let mut cpu = CPU::with_rom(rom);
    cpu.memory.ram[1] = 0x3;
    cpu.registers.x = 1;
    cpu.init();
    cpu.exec();
    assert_eq!(cpu.memory.ram[1], 3 << 1);
    assert_eq!(cpu.registers.negative, false);
    assert_eq!(cpu.registers.zero, false);
    assert_eq!(cpu.registers.carry, false);
}

#[test]
fn abs() {
    let mut rom = [0u8; 0x7fff];

    rom[0xFFFC - 0x8000] = 0x00;
    rom[0xFFFD - 0x8000] = 0x80;
    rom[0] = 0x0E;
    rom[1] = 0x11;
    rom[2] = 0x10;
    let mut cpu = CPU::with_rom(rom);
    cpu.memory.ram[0x1011] = 0x3;
    cpu.init();
    cpu.exec();
    assert_eq!(cpu.memory.ram[0x1011], 3 << 1);
    assert_eq!(cpu.registers.negative, false);
    assert_eq!(cpu.registers.zero, false);
    assert_eq!(cpu.registers.carry, false);
}

#[test]
fn abs_x() {
    let mut rom = [0u8; 0x7fff];

    rom[0xFFFC - 0x8000] = 0x00;
    rom[0xFFFD - 0x8000] = 0x80;
    rom[0] = 0x1E;
    rom[1] = 0x11;
    rom[2] = 0x10;
    let mut cpu = CPU::with_rom(rom);
    cpu.memory.ram[0x1012] = 0x3;
    cpu.registers.x = 1;
    cpu.init();
    cpu.exec();
    assert_eq!(cpu.memory.ram[0x1012], 3 << 1);
    assert_eq!(cpu.registers.negative, false);
    assert_eq!(cpu.registers.zero, false);
    assert_eq!(cpu.registers.carry, false);
}
