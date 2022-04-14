mod adc_instruction;
mod and_instruction;
mod lda_instruction;
mod ldx_instruction;
mod ldy_instruction;
mod ora_instruction;

use crate::CPU;

#[test]
fn init_cpu_sets_correct_pc() {
    let mut rom = [0u8; 0x7fff];

    rom[0xFFFC - 0x8000] = 0x00;
    rom[0xFFFD - 0x8000] = 0x80;
    let mut cpu = CPU::with_rom(rom);
    cpu.init();
    assert_eq!(
        cpu.registers.program_counter, 0x8000,
        "The PC register must be set to the address in 0xFFFC and 0xFFFD"
    );
}
#[test]
fn ram_write_test() {
    let mut cpu = CPU::new();
    cpu.write(4, 10);
    assert_eq!(
        cpu.memory.ram[4], 10,
        "The ram must hold the values written to it"
    );
}

#[test]
fn rom_write_test() {
    let mut cpu = CPU::new();
    cpu.write(0x8004, 10);
    assert_eq!(
        cpu.memory.rom[4], 0,
        "The rom must not change when a write request is made"
    );
}

#[test]
fn ram_read_test() {
    let mut cpu = CPU::new();
    cpu.memory.ram[4] = 10;
    assert_eq!(
        cpu.read(4),
        10,
        "The read method should return 10 from the ram after writing to it"
    );
}
#[test]
fn read_u16_test() {
    let mut rom = [0u8; 0x7fff];

    rom[0xFFFC - 0x8000] = 0x00;
    rom[0xFFFD - 0x8000] = 0x80;
    let cpu = CPU::with_rom(rom);
    assert_eq!(cpu.read_u16(0xFFFC), 0x8000)
}
