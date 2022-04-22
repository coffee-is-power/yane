use crate::Cartridge;
use std::rc::Rc;

#[derive(Copy, Clone)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}
pub struct PPU {
    //palette: [u32; 0x40],
    sprite_scr: [[Color; 256]; 240],
    //spr_name_table: [[[Color; 256]; 240]; 2],
    //spr_chr_table: [[[Color; 128]; 128]; 2],
    pub frame_complete: bool,
    scanline: i16,
    cycle: u16,
    cartridge: Rc<Cartridge>,
}
impl PPU {
    pub fn new(cartridge: Rc<Cartridge>) -> Self {
        Self {
            cartridge,
            scanline: 0,
            cycle: 0,
            frame_complete: false,
            sprite_scr: [[Color {
                r: 0,
                g: 0,
                b: 0,
                a: 0,
            }; 256]; 240],
        }
    }
    pub fn run(&mut self) {
        self.sprite_scr[self.scanline as usize][(self.cycle) as usize] = Color {
            r: 255,
            g: 255,
            b: 255,
            a: 255,
        };
        self.cycle += 1;
        if self.cycle >= 341 {
            self.cycle = 0;
            self.scanline += 1;
            if self.scanline >= 261 {
                self.scanline = -1;
                self.frame_complete = true;
            }
        }
    }
}
