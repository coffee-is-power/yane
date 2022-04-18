use crate::CPU;

#[test]
fn jsr() {
    let mut rom = [0u8; 0x7fff];

    rom[0xFFFC - 0x8000] = 0x00;
    rom[0xFFFD - 0x8000] = 0x80;
    rom[0] = 0x20;
    rom[1] = 0x50;
    rom[2] = 0x80;
    let mut cpu = CPU::with_rom(rom);
    cpu.init();
    cpu.exec();
    assert_eq!(cpu.registers.stack_pointer, 0x26);

    let hi = cpu.read(0x125) as u16;
    let lo = cpu.read(0x126) as u16;
    let addr = (hi << 8) | lo;
    assert_eq!(addr, 0x8003);
    assert_eq!(cpu.registers.program_counter, 0x8050);
}
#[test]
fn rts() {
    let mut rom = [0u8; 0x7fff];

    rom[0xFFFC - 0x8000] = 0x00;
    rom[0xFFFD - 0x8000] = 0x80;
    rom[0] = 0x20;
    rom[1] = 0x50;
    rom[2] = 0x80;
    rom[0x50] = 0x60;
    let mut cpu = CPU::with_rom(rom);
    cpu.init();
    cpu.exec();
    cpu.exec();
    assert_eq!(cpu.registers.stack_pointer, 0x24);
    assert_eq!(cpu.registers.program_counter, 0x8003);
}
