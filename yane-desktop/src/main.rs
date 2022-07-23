use std::{
    cell::RefCell,
    rc::Rc,
    sync::{Arc, Mutex, MutexGuard},
};
use yane_core::*;

use std::collections::VecDeque;
use std::time::{Duration, Instant};

/// Measures Frames Per Second (FPS).
#[derive(Debug)]
pub struct FPSCounter {
    /// The last registered frames.
    last_second: Instant,
    frames: usize,
    pub last_second_frames: usize,
}

impl Default for FPSCounter {
    fn default() -> Self {
        FPSCounter::new()
    }
}

impl FPSCounter {
    /// Creates a new FPSCounter.
    pub fn new() -> FPSCounter {
        FPSCounter {
            last_second: Instant::now(),
            frames: 0,
            last_second_frames: 0,
        }
    }
    /// Updates the FPSCounter and returns number of frames.
    pub fn tick(&mut self) -> usize {
        self.frames += 1;
        if self.last_second.elapsed() >= Duration::from_secs(1) {
            self.last_second = Instant::now();
            self.last_second_frames = self.frames;
            self.frames = 0;
        }
        return self.last_second_frames;
    }
}
fn main() {
    let cartridge = Rc::new(RefCell::new(get_cartridge()));
    let ppu = Rc::new(RefCell::new(PPU::new(&cartridge)));
    let memory = Memory::new(&cartridge, &ppu);
    let mut cpu = CPU::new(memory);
    let mut clock_counter = 0;

    let mut last_second: Instant = Instant::now();
    cpu.init();
    // let mut window = simple::Window::new("Yane for Desktop", 256, 240);
    let mut cps_counter = FPSCounter::new();
    loop {
        // Create before loop Instant
        cpu.clock();
        //if window.next_frame() {
        //draw_chr_memory(&mut window, 0, 1, 0, 0, &ppu_ref);
        //draw_chr_mem28, 0, &ppu_ref);
        //} else {
        //  return;
        //}
        let cps = cps_counter.tick();
        if last_second.elapsed() >= Duration::from_secs(1) {
            // Warning: CPU is slow
            println!("Debug: Clock: {}", cps);
            last_second = Instant::now();
        }
    }
}
// fn draw_chr_memory(
//     window: &mut Window,
//     i: u8,
//     palette: u8,
//     offset_x: i32,
//     offset_y: i32,
//     ppu: &PPU,
// ) {
//     let colors = ppu.get_pattern_tables(i, palette);
//     for x in 0..128i32 {
//         for y in 0..128i32 {
//             let color = colors[y as usize][x as usize];
//             window.set_color(color.r, color.g, color.b, 255);
//             window.draw_point(Point::new(x + offset_x, y + offset_y));
//         }
//     }
// }
static DEFAULT_ROM: &str = "./test-roms/nestest.nes";
fn get_cartridge() -> Cartridge {
    let args_iter = std::env::args();
    let args: Vec<String> = args_iter.collect();
    let default_rom_string: String = String::from(DEFAULT_ROM);
    let path = args.get(1).or_else(|| Some(&default_rom_string)).unwrap();
    Cartridge::from_file(path).unwrap()
}
