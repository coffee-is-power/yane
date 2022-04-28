#![feature(get_mut_unchecked)]
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
use raylib::prelude::RaylibDrawHandle;
use raylib::{RaylibHandle, RaylibThread};
use std::rc::Rc;

fn main() {
    let cartridge = Rc::new(get_cartridge());
    
    let mut ppu_rc = Rc::new(PPU::new(cartridge.clone()));
    let ppu =unsafe{ Rc::get_mut_unchecked(&mut ppu_rc)};
    let clone_ppu = || {
        unsafe{Rc::from_raw(ppu)}
    };
    let memory = Memory::new(Rc::clone(&cartridge), clone_ppu());
    let mut cpu = CPU::new(Rc::new(memory));
    let mut clock_counter = 0;
    let (mut rl, rl_thread) = init_raylib();
    cpu.init();
    while !rl.window_should_close() {
        clock_counter = clock(clock_counter, &mut cpu, ppu);
        let mut d = rl.begin_drawing(&rl_thread);
        draw_chr_memory(&mut d, 0, 1, 0, 0, ppu);
        draw_chr_memory(&mut d, 1, 1, 128, 0, ppu);
        
    }
}
fn draw_chr_memory(d: &mut RaylibDrawHandle, i: u8, palette: u8, offset_x: i32, offset_y: i32, ppu: &PPU){
    let colors = ppu.get_pattern_tables(i, palette);
        for x in 0..128i32 {
            for y in 0..128i32 {
                let color = colors[y as usize][x as usize];
                d.draw_pixel(x + offset_x, y + offset_y, Color::new(color.r, color.g, color.b, 255));
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
static DEFAULT_ROM: &str = "./test-roms/nestest.nes";
fn get_cartridge() -> Cartridge {
    let args_iter = std::env::args();
    let args: Vec<String> = args_iter.collect();
    let default_rom_string: String = String::from(DEFAULT_ROM);
    let path = args.get(1).or_else(||Some(&default_rom_string)).unwrap();
    Cartridge::from_file(path).unwrap()
}
fn init_raylib() -> (RaylibHandle, RaylibThread) {
    raylib::init()
        .title("Yet Another NES Emulator")
        .size(256, 240)
        .build()
}
