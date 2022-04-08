mod cpu;
mod memory;
use cpu::CPU;
fn main() {
    let mut cpu = CPU::new();
    cpu.write(0xFFFD, 0x80);/* set entrypoint */
    cpu.write(0xFFFC, 0x0);
    cpu.write(0x8000, 0xAE);/* ORA */
    cpu.write(0x8001, 0x7);
    cpu.write(7, 4);
    cpu.init();
    cpu.exec();
    println!("{:#?}", cpu.registers);
}
