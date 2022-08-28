use crate::CPU;#[test]
fn immediate() {
    let mut rom = [0u8; 0x7fff];

    rom[0x3FFC] = 0x00;
    rom[0x3FFD] = 0x80;
    rom[0] = 0x69;
    rom[1] = 10;
    rom[2] = 0x69;
    rom[3] = 7;



    let mut cpu = CPU::from_rom(&rom);
    cpu.init();
    cpu.exec();
    cpu.exec();
    assert_eq!(cpu.registers.a, 17);
}

#[test]
fn zeropage() {
    let mut rom = [0u8; 0x7fff];

    rom[0x3FFC] = 0x00;
    rom[0x3FFD] = 0x80;
    rom[0] = 0x65;
    rom[1] = 60;
    rom[2] = 0x65;
    rom[3] = 61;


    let mut cpu = CPU::from_rom(&rom);
    cpu.write(60, 10);
    cpu.write(61, 7);
    cpu.init();
    cpu.exec();
    cpu.exec();
    assert_eq!(cpu.registers.a, 17);
}

#[test]
fn zeropage_x() {
    let mut rom = [0u8; 0x7fff];

    rom[0x3FFC] = 0x00;
    rom[0x3FFD] = 0x80;
    rom[0] = 0x75;
    rom[1] = 60;
    rom[2] = 0x75;
    rom[3] = 61;

    let mut cpu = CPU::from_rom(&rom);
    cpu.registers.x = 1;
    cpu.write(61, 10);
    cpu.write(62, 7);
    cpu.init();
    cpu.exec();
    cpu.exec();
    assert_eq!(cpu.registers.a, 17);
}

#[test]
fn abs() {
    let mut rom = [0u8; 0x7fff];

    rom[0x3FFC] = 0x00;
    rom[0x3FFD] = 0x80;
    rom[0] = 0x6D;
    rom[1] = 0x10;
    rom[2] = 0x10;

    rom[3] = 0x6D;
    rom[4] = 0x11;
    rom[5] = 0x10;
    let mut cpu = CPU::from_rom(&rom);
    cpu.write(0x1010, 10);
    cpu.write(0x1011, 7);
    cpu.init();
    cpu.exec();
    cpu.exec();
    assert_eq!(cpu.registers.a, 17);
}

#[test]
fn abs_x() {
    let mut rom = [0u8; 0x7fff];

    rom[0x3FFC] = 0x00;
    rom[0x3FFD] = 0x80;
    rom[0] = 0x7D;
    rom[1] = 0x10;
    rom[2] = 0x10;
    rom[3] = 0x7D;
    rom[4] = 0x11;
    rom[5] = 0x10;
    let mut cpu = CPU::from_rom(&rom);
    cpu.registers.x = 1;
    cpu.write(0x1011, 10);
    cpu.write(0x1012, 7);
    cpu.init();
    cpu.exec();
    cpu.exec();
    assert_eq!(cpu.registers.a, 17);
}

#[test]
fn abs_y() {
    let mut rom = [0u8; 0x7fff];

    rom[0x3FFC] = 0x00;
    rom[0x3FFD] = 0x80;
    rom[0] = 0x79;
    rom[1] = 0x10;
    rom[2] = 0x10;

    rom[3] = 0x79;
    rom[4] = 0x11;
    rom[5] = 0x10;
    let mut cpu = CPU::from_rom(&rom);
    cpu.registers.y = 1;
    cpu.write(0x1011, 10);
    cpu.write(0x1012, 7);
    cpu.init();
    cpu.exec();
    cpu.exec();
    assert_eq!(cpu.registers.a, 17);
}

#[test]
fn indirect_y() {
    let mut rom = [0u8; 0x7fff];

    rom[0x3FFC] = 0x00;
    rom[0x3FFD] = 0x80;
    rom[0] = 0x71;
    rom[1] = 0x11;

    rom[2] = 0x71;
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
    assert_eq!(cpu.registers.a, 17);
}

#[test]
fn indirect_x() {
    let mut rom = [0u8; 0x7fff];

    rom[0x3FFC] = 0x00;
    rom[0x3FFD] = 0x80;
    rom[0] = 0x61;
    rom[1] = 0x11;

    rom[2] = 0x61;
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
    assert_eq!(cpu.registers.a, 17);
}

#[test]
fn overflow_is_on_when_overflows() {
    let mut rom = [0u8; 0x7fff];

    rom[0x3FFC] = 0x00;
    rom[0x3FFD] = 0x80;
    rom[0] = 0x69;
    rom[1] = 0x7f;
    rom[2] = 0x69;
    rom[3] = 1;
    let mut cpu = CPU::from_rom(&rom);
    cpu.init();
    cpu.exec();
    cpu.exec();

    assert_eq!(cpu.registers.a, 0x80);
    assert!(cpu.registers.overflow);//0x7f + 1 wraps to -127
}
#[test]
fn carry_is_on_when_wraps() {
    let mut rom = [0u8; 0x7fff];

    rom[0x3FFC] = 0x00;
    rom[0x3FFD] = 0x80;
    rom[0] = 0x69;
    rom[1] = 0xFF;
    rom[2] = 0x69;
    rom[3] = 0xFF;
let mut cpu = CPU::from_rom(&rom);
    cpu.init();
    cpu.exec();
    cpu.exec();

    assert_eq!(cpu.registers.a, 0xFE);
    assert!(cpu.registers.carry);
}
#[test]
fn adds_carry_to_sum() {
    let mut rom = [0u8; 0x7fff];

    rom[0x3FFC] = 0x00;
    rom[0x3FFD] = 0x80;
    rom[0] = 0x69;
    rom[1] = 0xF0;
    rom[2] = 0x69;
    rom[3] = 2;
let mut cpu = CPU::from_rom(&rom);
    cpu.registers.carry = true;
    cpu.init();
    cpu.exec();
    cpu.exec();

    assert_eq!(cpu.registers.a, 0xf3);
    assert_eq!(cpu.registers.carry, false);
}
