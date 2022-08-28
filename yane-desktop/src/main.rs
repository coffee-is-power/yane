use simple::{Window, Point};
use yane_core::*;

fn main() {
    let cartridge = get_cartridge();
    let mut nes = NES::new(cartridge);
    let mut window = simple::Window::new("Yane for Desktop", 256, 240);
    loop {
        nes.clock();
        let ppu = nes.ppu();
        if ppu.frame_complete && window.next_frame() {
            draw_chr_memory(&mut window, 0, 1, 0, 0, &ppu);
            draw_chr_memory(&mut window, 1, 1, 128, 0, &ppu);
        }
    }
}
fn draw_chr_memory(window: &mut Window, i: u8, palette: u8, offset_x: i32, offset_y: i32, ppu: &PPU){
    let colors = ppu.get_pattern_tables(i, palette);
        for x in 0..128i32 {
            for y in 0..128i32 {
                let color = colors[y as usize][x as usize];
                window.set_color(color.r, color.g, color.b, 255);
                window.draw_point(Point::new(x + offset_x, y + offset_y));
            }
        }
}
static DEFAULT_ROM: &str = "./test-roms/nestest.nes";
fn get_cartridge() -> Cartridge {
    let args_iter = std::env::args();
    let args: Vec<String> = args_iter.collect();
    let default_rom_string: String = String::from(DEFAULT_ROM);
    let path = args.get(1).or_else(||Some(&default_rom_string)).unwrap();
    Cartridge::from_file(path).unwrap()
}
