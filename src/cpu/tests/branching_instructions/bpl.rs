use crate::CPU;

#[test]
fn bpl_jumps_when_negative_is_false() {
    let mut rom = [0u8; 0x7fff];
    rom[0xFFFC - 0x8000] = 0x00;
    rom[0xFFFD - 0x8000] = 0x80;
    rom[0] = 0x10;
    rom[1] = 4;
    let mut cpu = CPU::with_rom(rom);
    cpu.init();
    cpu.registers.negative = false;
    cpu.exec();
    assert_eq!(cpu.registers.program_counter, 0x8005);
}

#[test]
fn bpl_does_not_jump_when_negative_is_true() {
    let mut rom = [0u8; 0x7fff];
    rom[0xFFFC - 0x8000] = 0x00;
    rom[0xFFFD - 0x8000] = 0x80;
    rom[0] = 0x10;
    rom[1] = 4;
    let mut cpu = CPU::with_rom(rom);
    cpu.init();
    cpu.registers.negative = true;
    cpu.exec();
    assert_eq!(cpu.registers.program_counter, 0x8001);
}
