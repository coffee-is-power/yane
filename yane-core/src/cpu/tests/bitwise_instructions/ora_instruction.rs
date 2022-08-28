use crate::CPU;#[test]
fn immediate_instruction() {
    let mut rom = [0u8; 0x7fff];

    rom[0x3FFC] = 0x00;
    rom[0x3FFD] = 0x80;
    rom[0] = 0x09;
    rom[1] = 10;
    rom[2] = 0x09;
    rom[3] = 7;

    let mut cpu = CPU::from_rom(&rom);
    cpu.init();
    cpu.exec();
    cpu.exec();
    assert_eq!(cpu.registers.a, 15);
}

#[test]
fn zeropage() {
    let mut rom = [0u8; 0x7fff];

    rom[0x3FFC] = 0x00;
    rom[0x3FFD] = 0x80;
    rom[0] = 0x05;
    rom[1] = 60;
    rom[2] = 0x05;
    rom[3] = 61;


let mut cpu = CPU::from_rom(&rom);
    cpu.write(60, 10);
    cpu.write(61, 7);
    cpu.init();
    cpu.exec();
    cpu.exec();
    assert_eq!(cpu.registers.a, 15);
}

#[test]
fn zeropage_x() {
    let mut rom = [0u8; 0x7fff];

    rom[0x3FFC] = 0x00;
    rom[0x3FFD] = 0x80;
    rom[0] = 0x15;
    rom[1] = 60;
    rom[2] = 0x15;
    rom[3] = 61;


let mut cpu = CPU::from_rom(&rom);
    cpu.registers.x = 1;
    cpu.write(61, 10);
    cpu.write(62, 7);
    cpu.init();
    cpu.exec();
    cpu.exec();
    assert_eq!(cpu.registers.a, 15);
}

#[test]
fn abs() {
    let mut rom = [0u8; 0x7fff];

    rom[0x3FFC] = 0x00;
    rom[0x3FFD] = 0x80;
    rom[0] = 0xD;
    rom[1] = 0x10;
    rom[2] = 0x10;

    rom[3] = 0xD;
    rom[4] = 0x11;
    rom[5] = 0x10;


let mut cpu = CPU::from_rom(&rom);
    cpu.write(0x1010, 10);
    cpu.write(0x1011, 7);
    cpu.init();
    cpu.exec();
    cpu.exec();
    assert_eq!(cpu.registers.a, 15);
}

#[test]
fn abs_x() {
    let mut rom = [0u8; 0x7fff];

    rom[0x3FFC] = 0x00;
    rom[0x3FFD] = 0x80;
    rom[0] = 0x1D;
    rom[1] = 0x10;
    rom[2] = 0x10;
    rom[3] = 0x1D;
    rom[4] = 0x11;
    rom[5] = 0x10;


let mut cpu = CPU::from_rom(&rom);
    cpu.registers.x = 1;
    cpu.write(0x1011, 10);
    cpu.write(0x1012, 7);
    cpu.init();
    cpu.exec();
    cpu.exec();
    assert_eq!(cpu.registers.a, 15);
}

#[test]
fn abs_y() {
    let mut rom = [0u8; 0x7fff];

    rom[0x3FFC] = 0x00;
    rom[0x3FFD] = 0x80;
    rom[0] = 0x19;
    rom[1] = 0x10;
    rom[2] = 0x10;

    rom[3] = 0x19;
    rom[4] = 0x11;
    rom[5] = 0x10;


let mut cpu = CPU::from_rom(&rom);
    cpu.registers.y = 1;
    cpu.write(0x1011, 10);
    cpu.write(0x1012, 7);
    cpu.init();
    cpu.exec();
    cpu.exec();
    assert_eq!(cpu.registers.a, 15);
}

#[test]
fn indirect_y() {
    let mut rom = [0u8; 0x7fff];

    rom[0x3FFC] = 0x00;
    rom[0x3FFD] = 0x80;
    rom[0] = 0x11;
    rom[1] = 0x11;

    rom[2] = 0x11;
    rom[3] = 0x11;


let mut cpu = CPU::from_rom(&rom);
    cpu.registers.y = 1;
    cpu.write(0x11, 0x11);
    cpu.write(0x12, 0x12);
    cpu.write(0x1212, 10);
    cpu.init();
    cpu.exec();
    cpu.write(0x1212, 7);
    cpu.exec();
    assert_eq!(cpu.registers.a, 15);
}

#[test]
fn indirect_x() {
    let mut rom = [0u8; 0x7fff];

    rom[0x3FFC] = 0x00;
    rom[0x3FFD] = 0x80;
    rom[0] = 0x1;
    rom[1] = 0x11;

    rom[2] = 0x1;
    rom[3] = 0x11;


let mut cpu = CPU::from_rom(&rom);
    cpu.registers.x = 1;
    cpu.write(0x12, 0x11);
    cpu.write(0x13, 0x12);
    cpu.write(0x1211, 10);
    cpu.init();
    cpu.exec();
    cpu.write(0x1211, 7);
    cpu.exec();
    assert_eq!(cpu.registers.a, 15);
}
