use crate::cpu::registers::Registers;
use crate::CPU;

#[test]
fn push() {
    let mut cpu = CPU::new();
    cpu.push(10);
    assert_eq!(
        cpu.registers.stack_pointer, 0xFE,
        "The sp must be decremented"
    );
    assert_eq!(
        cpu.read(0x1FE),
        10,
        "The pushed value should be written to memory"
    );
}

#[test]
fn pop() {
    let mut cpu = CPU::new();
    cpu.push(10);

    let value = cpu.pop();
    assert_eq!(
        cpu.registers.stack_pointer, 0xFF,
        "The sp must be incremented"
    );
    assert_eq!(
        cpu.read(0xFE),
        0,
        "The popped value should be deleted from memory"
    );
    assert_eq!(value, 10);
}
