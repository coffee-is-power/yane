use crate::CPU;

#[test]
fn bvc_jumps_when_overflow_is_false() {
    let mut rom = [0u8; 0x7fff];
    rom[0xFFFC - 0x8000] = 0x00;
    rom[0xFFFD - 0x8000] = 0x80;
    rom[0] = 0x50;
    rom[1] = 4;
    let mut cpu = CPU::with_rom(rom);
    cpu.init();
    cpu.registers.overflow = false;
    cpu.exec();
    assert_eq!(cpu.registers.program_counter, 0x8005);
}

#[test]
fn bvc_does_not_jump_when_overflow_is_true() {
    let mut rom = [0u8; 0x7fff];
    rom[0xFFFC - 0x8000] = 0x00;
    rom[0xFFFD - 0x8000] = 0x80;
    rom[0] = 0x50;
    rom[1] = 4;
    let mut cpu = CPU::with_rom(rom);
    cpu.init();
    cpu.registers.overflow = true;
    cpu.exec();
    assert_eq!(cpu.registers.program_counter, 0x8001);
}
