use crate::CPU;

#[test]
fn bit_zp() {
    let mut rom = [0u8; 0x7fff];

    rom[0xFFFC - 0x8000] = 0x00;
    rom[0xFFFD - 0x8000] = 0x80;
    rom[0] = 0x24;
    rom[1] = 0x10;
    let mut cpu = CPU::with_rom(rom);
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

    rom[0xFFFC - 0x8000] = 0x00;
    rom[0xFFFD - 0x8000] = 0x80;

    rom[0] = 0x2C;
    rom[1] = 0x10;
    rom[2] = 0x10;
    let mut cpu = CPU::with_rom(rom);
    cpu.registers.a = 0xF;
    cpu.write(0x1010, 0b11000000);
    cpu.init();
    cpu.exec();
    assert!(cpu.registers.zero);
    assert!(cpu.registers.negative);
    assert!(cpu.registers.overflow);
}
