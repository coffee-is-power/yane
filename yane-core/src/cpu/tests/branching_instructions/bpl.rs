use crate::CPU;
#[test]
fn bpl_jumps_when_negative_is_false() {
    let mut rom = [0u8; 0x7fff];
    rom[0x3FFC] = 0x00;
    rom[0x3FFD] = 0x80;
    rom[0] = 0x10;
    rom[1] = 3;


let mut cpu = CPU::from_rom(&rom);
    cpu.init();
    cpu.registers.negative = false;
    cpu.exec();
    assert_eq!(cpu.registers.program_counter, 0x8002 + 3);
}

#[test]
fn bpl_does_not_jump_when_negative_is_true() {
    let mut rom = [0u8; 0x7fff];
    rom[0x3FFC] = 0x00;
    rom[0x3FFD] = 0x80;
    rom[0] = 0x10;
    rom[1] = 3;


let mut cpu = CPU::from_rom(&rom);
    cpu.init();
    cpu.registers.negative = true;
    cpu.exec();
    assert_eq!(cpu.registers.program_counter, 0x8002);
}

#[test]
fn bpl_works_with_negative_relative_addresses() {
    let mut rom = [0u8; 0x7fff];
    rom[0x3FFC] = 0x00;
    rom[0x3FFD] = 0x80;
    rom[0] = 0x10;
    rom[1] = -6i8 as u8;


let mut cpu = CPU::from_rom(&rom);
    cpu.init();
    cpu.registers.negative = false;
    cpu.exec();
    assert_eq!(cpu.registers.program_counter, 0x8002-6);
}