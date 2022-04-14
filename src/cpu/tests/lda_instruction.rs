use crate::CPU;
#[test]
fn immediate() {
    let mut rom = [0u8; 0x7fff];

    rom[0xFFFC - 0x8000] = 0x00;
    rom[0xFFFD - 0x8000] = 0x80;
    rom[0] = 0xA9;
    rom[1] = 0x10;
    let mut cpu = CPU::with_rom(rom);
    cpu.init();
    cpu.exec();
    assert_eq!(cpu.registers.a, 0x10);
}

#[test]
fn zero_page() {
    let mut rom = [0u8; 0x7fff];

    rom[0xFFFC - 0x8000] = 0x00;
    rom[0xFFFD - 0x8000] = 0x80;
    rom[0] = 0xA5;
    rom[1] = 0x10;
    let mut cpu = CPU::with_rom(rom);
    cpu.write(0x10, 0x4);
    cpu.init();
    cpu.exec();
    assert_eq!(cpu.registers.a, 0x4);
}

#[test]
fn zero_page_x() {
    let mut rom = [0u8; 0x7fff];

    rom[0xFFFC - 0x8000] = 0x00;
    rom[0xFFFD - 0x8000] = 0x80;
    rom[0] = 0xB5;
    rom[1] = 0x10;
    let mut cpu = CPU::with_rom(rom);
    cpu.registers.x = 2;
    cpu.write(0x12, 0x4);
    cpu.init();
    cpu.exec();
    assert_eq!(cpu.registers.a, 0x4);
}

#[test]
fn abs() {
    let mut rom = [0u8; 0x7fff];

    rom[0xFFFC - 0x8000] = 0x00;
    rom[0xFFFD - 0x8000] = 0x80;
    rom[0] = 0xAD;
    rom[1] = 0x00;
    rom[2] = 0x10;
    let mut cpu = CPU::with_rom(rom);
    cpu.write(0x1000, 0x4);
    cpu.init();
    cpu.exec();
    assert_eq!(cpu.registers.a, 0x4);
}

#[test]
fn abs_x() {
    let mut rom = [0u8; 0x7fff];

    rom[0xFFFC - 0x8000] = 0x00;
    rom[0xFFFD - 0x8000] = 0x80;
    rom[0] = 0xBD;
    rom[1] = 0x00;
    rom[2] = 0x10;
    let mut cpu = CPU::with_rom(rom);
    cpu.registers.x = 2;
    cpu.write(0x1002, 0x4);
    cpu.init();
    cpu.exec();
    assert_eq!(cpu.registers.a, 0x4);
}

#[test]
fn abs_y() {
    let mut rom = [0u8; 0x7fff];

    rom[0xFFFC - 0x8000] = 0x00;
    rom[0xFFFD - 0x8000] = 0x80;
    rom[0] = 0xB9;
    rom[1] = 0x00;
    rom[2] = 0x10;
    let mut cpu = CPU::with_rom(rom);
    cpu.registers.y = 2;
    cpu.write(0x1002, 0x4);
    cpu.init();
    cpu.exec();
    assert_eq!(cpu.registers.a, 0x4);
}

#[test]
fn indirect_x() {
    let mut rom = [0u8; 0x7fff];

    rom[0xFFFC - 0x8000] = 0x00;
    rom[0xFFFD - 0x8000] = 0x80;
    rom[0] = 0xA1;
    rom[1] = 0x10;
    let mut cpu = CPU::with_rom(rom);
    cpu.registers.x = 2;
    cpu.write(0x12, 0x00);
    cpu.write(0x13, 0x04);
    cpu.write(0x400, 4);
    cpu.init();
    cpu.exec();
    assert_eq!(cpu.registers.a, 0x4);
}

#[test]
fn indirect_y() {
    let mut rom = [0u8; 0x7fff];

    rom[0xFFFC - 0x8000] = 0x00;
    rom[0xFFFD - 0x8000] = 0x80;
    rom[0] = 0xB1;
    rom[1] = 0x10;
    let mut cpu = CPU::with_rom(rom);
    cpu.registers.y = 2;
    cpu.write(0x10, 0x00);
    cpu.write(0x11, 0x04);
    cpu.write(0x402, 4);
    cpu.init();
    cpu.exec();
    assert_eq!(cpu.registers.a, 0x4);
}