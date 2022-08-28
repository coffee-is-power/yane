use crate::CPU;
#[test]
fn tax() {
    let mut rom = [0u8; 0x7fff];
    rom[0x3FFC] = 0x00;
    rom[0x3FFD] = 0x80;
    rom[0] = 0xAA;
    let mut cpu = CPU::from_rom(&rom);
    cpu.init();
    cpu.registers.a = 10;
    cpu.exec();
    assert_eq!(cpu.registers.a, 10);
    assert_eq!(cpu.registers.x, 10);
}
#[test]
fn txa() {
    let mut rom = [0u8; 0x7fff];

    rom[0x3FFC] = 0x00;
    rom[0x3FFD] = 0x80;
    rom[0] = 0x8A;


let mut cpu = CPU::from_rom(&rom);
    cpu.init();
    cpu.registers.x = 10;
    cpu.exec();
    assert_eq!(cpu.registers.a, 10);
    assert_eq!(cpu.registers.x, 10);
}

#[test]
fn tay() {
    let mut rom = [0u8; 0x7fff];

    rom[0x3FFC] = 0x00;
    rom[0x3FFD] = 0x80;
    rom[0] = 0xA8;


let mut cpu = CPU::from_rom(&rom);
    cpu.init();
    cpu.registers.a = 10;
    cpu.exec();
    assert_eq!(cpu.registers.a, 10);
    assert_eq!(cpu.registers.y, 10);
}

#[test]
fn tya() {
    let mut rom = [0u8; 0x7fff];

    rom[0x3FFC] = 0x00;
    rom[0x3FFD] = 0x80;
    rom[0] = 0x98;


let mut cpu = CPU::from_rom(&rom);
    cpu.init();
    cpu.registers.y = 10;
    cpu.exec();
    assert_eq!(cpu.registers.y, 10);
    assert_eq!(cpu.registers.a, 10);
}
