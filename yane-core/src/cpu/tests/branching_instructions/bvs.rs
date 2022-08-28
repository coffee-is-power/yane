use crate::CPU;
#[test]
fn bvs_jumps_when_overflow_is_true() {
    let mut rom = [0u8; 0x7fff];
    rom[0x3FFC] = 0x00;
    rom[0x3FFD] = 0x80;
    rom[0] = 0x70;
    rom[1] = 4;


let mut cpu = CPU::from_rom(&rom);
    cpu.init();
    cpu.registers.overflow = true;
    cpu.exec();
    assert_eq!(cpu.registers.program_counter, 0x8006);
}

#[test]
fn bvs_does_not_jump_when_overflow_is_false() {
    let mut rom = [0u8; 0x7fff];
    rom[0x3FFC] = 0x00;
    rom[0x3FFD] = 0x80;
    rom[0] = 0x70;
    rom[1] = 4;


let mut cpu = CPU::from_rom(&rom);
    cpu.init();
    cpu.registers.overflow = false;
    cpu.exec();
    assert_eq!(cpu.registers.program_counter, 0x8002);
}
