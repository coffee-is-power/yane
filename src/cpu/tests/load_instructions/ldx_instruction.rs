use crate::CPU;
#[test]
fn immediate() {
    let mut rom = [0u8; 0x7fff];

    rom[0xFFFC - 0x8000] = 0x00;
    rom[0xFFFD - 0x8000] = 0x80;
    rom[0] = 0xA2;
    rom[1] = 0x10;
    let mut cpu = CPU::with_rom(rom);
    cpu.init();
    cpu.exec();
    assert_eq!(cpu.registers.x, 0x10);
}

#[test]
fn zero_page() {
    let mut rom = [0u8; 0x7fff];

    rom[0xFFFC - 0x8000] = 0x00;
    rom[0xFFFD - 0x8000] = 0x80;
    rom[0] = 0xA6;
    rom[1] = 0x10;
    let mut cpu = CPU::with_rom(rom);
    cpu.write(0x10, 0x4);
    cpu.init();
    cpu.exec();
    assert_eq!(cpu.registers.x, 0x4);
}

#[test]
fn zero_page_y() {
    let mut rom = [0u8; 0x7fff];

    rom[0xFFFC - 0x8000] = 0x00;
    rom[0xFFFD - 0x8000] = 0x80;
    rom[0] = 0xB6;
    rom[1] = 0x10;
    let mut cpu = CPU::with_rom(rom);
    cpu.registers.y = 2;
    cpu.write(0x12, 0x4);
    cpu.init();
    cpu.exec();
    assert_eq!(cpu.registers.x, 0x4);
}

#[test]
fn abs() {
    let mut rom = [0u8; 0x7fff];

    rom[0xFFFC - 0x8000] = 0x00;
    rom[0xFFFD - 0x8000] = 0x80;
    rom[0] = 0xAE;
    rom[1] = 0x00;
    rom[2] = 0x10;
    let mut cpu = CPU::with_rom(rom);
    cpu.write(0x1000, 0x4);
    cpu.init();
    cpu.exec();
    assert_eq!(cpu.registers.x, 0x4);
}

#[test]
fn abs_y() {
    let mut rom = [0u8; 0x7fff];

    rom[0xFFFC - 0x8000] = 0x00;
    rom[0xFFFD - 0x8000] = 0x80;
    rom[0] = 0xBE;
    rom[1] = 0x00;
    rom[2] = 0x10;
    let mut cpu = CPU::with_rom(rom);
    cpu.registers.y = 2;
    cpu.write(0x1002, 0x4);
    cpu.init();
    cpu.exec();
    assert_eq!(cpu.registers.x, 0x4);
}
