extern crate core;

mod cartridge;
mod cpu;
mod mapper;
mod mapper_0;
mod memory;

use crate::cartridge::Cartridge;
use cpu::CPU;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::os::unix::fs::MetadataExt;

fn main() -> ! {
    let args_iter = std::env::args();
    let args: Vec<String> = args_iter.collect();
    let path = args.get(1).unwrap();
    let mut cartridge = Cartridge::from_file(path).unwrap();
    let mut cpu = CPU::new(&mut cartridge);
    cpu.init();
    loop {
        cpu.exec();
    }
}
