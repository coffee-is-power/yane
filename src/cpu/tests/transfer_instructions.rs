use crate::CPU;

#[test]
fn tax() {
    let mut rom = [0u8; 0x7fff];

    rom[0xFFFC - 0x8000] = 0x00;
    rom[0xFFFD - 0x8000] = 0x80;
    rom[0] = 0xAA;
    let mut cpu = CPU::with_rom(rom);
    cpu.init();
    cpu.registers.x = 10;
    cpu.exec();
    assert_eq!(cpu.registers.x, 10);
    assert_eq!(cpu.registers.a, 10);
}
#[test]
fn txa() {
    let mut rom = [0u8; 0x7fff];

    rom[0xFFFC - 0x8000] = 0x00;
    rom[0xFFFD - 0x8000] = 0x80;
    rom[0] = 0x8A;
    let mut cpu = CPU::with_rom(rom);
    cpu.init();
    cpu.registers.a = 10;
    cpu.exec();
    assert_eq!(cpu.registers.x, 10);
    assert_eq!(cpu.registers.a, 10);
}

#[test]
fn tay() {
    let mut rom = [0u8; 0x7fff];

    rom[0xFFFC - 0x8000] = 0x00;
    rom[0xFFFD - 0x8000] = 0x80;
    rom[0] = 0xA8;
    let mut cpu = CPU::with_rom(rom);
    cpu.init();
    cpu.registers.a = 10;
    cpu.exec();
    assert_eq!(cpu.registers.a, 10);
    assert_eq!(cpu.registers.y, 10);
}

#[test]
fn tya() {
    let mut rom = [0u8; 0x7fff];

    rom[0xFFFC - 0x8000] = 0x00;
    rom[0xFFFD - 0x8000] = 0x80;
    rom[0] = 0x98;
    let mut cpu = CPU::with_rom(rom);
    cpu.init();
    cpu.registers.y = 10;
    cpu.exec();
    assert_eq!(cpu.registers.y, 10);
    assert_eq!(cpu.registers.a, 10);
}
