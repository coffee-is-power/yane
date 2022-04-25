extern crate core;

mod cartridge;
mod cpu;
mod mapper;
mod mapper_0;
mod memory;
mod ppu;

use crate::cartridge::Cartridge;
use crate::memory::Memory;
use crate::ppu::PPU;
use cpu::CPU;
use raylib::color::Color;
use raylib::drawing::RaylibDraw;
use raylib::{RaylibHandle, RaylibThread};
use std::rc::Rc;

fn main() {
    let cartridge = Rc::new(get_cartridge());

    let memory = Rc::new(Memory::new(cartridge.clone()));
    let mut cpu = CPU::new(Rc::clone(&memory));
    let mut ppu = PPU::new(cartridge.clone());
    let mut clock_counter = 0;
    let (mut rl, rl_thread) = init_raylib();
    cpu.init();
    while !rl.window_should_close() {
        clock_counter = clock(clock_counter, &mut cpu, &mut ppu);
        let mut d = rl.begin_drawing(&rl_thread);
        let colors = ppu.get_pattern_tables(0, 1);
        for x in 0..128i32 {
            for y in 0..128i32 {
                let color = colors[y as usize][x as usize];
                d.draw_pixel(x, y, Color::new(color.r, color.g, color.b, 255));
            }
        }
    }
}
fn clock(clock_counter: u32, cpu: &mut CPU, ppu: &mut PPU) -> u32 {
    if (clock_counter % 3) == 0 {
        cpu.clock();
    }
    ppu.run();
    clock_counter + 1
}
fn get_cartridge() -> Cartridge {
    let args_iter = std::env::args();
    let args: Vec<String> = args_iter.collect();
    let path = args.get(1).unwrap();
    Cartridge::from_file(path).unwrap()
}
fn init_raylib() -> (RaylibHandle, RaylibThread) {
    raylib::init()
        .title("Yet Another NES Emulator")
        .size(256, 240)
        .build()
}
