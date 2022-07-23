use crate::cartridge::Cartridge;
use std::{
    cell::RefCell,
    rc::Rc,
    sync::{Arc, Mutex},
};

#[derive(Copy, Clone)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}
impl Color {
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    pub fn from_scalar(s: u8) -> Self {
        Self::new(s, s, s)
    }
}
static PPU_BACKGROUND_COLORS_MAP: [Color; 0x40] = [
    Color::new(84, 84, 84),
    Color::new(0, 30, 116),
    Color::new(8, 16, 144),
    Color::new(48, 0, 136),
    Color::new(68, 0, 100),
    Color::new(92, 0, 48),
    Color::new(84, 4, 0),
    Color::new(60, 24, 0),
    Color::new(32, 42, 0),
    Color::new(8, 58, 0),
    Color::new(0, 64, 0),
    Color::new(0, 60, 0),
    Color::new(0, 50, 60),
    Color::new(0, 0, 0),
    Color::new(0, 0, 0),
    Color::new(0, 0, 0),
    Color::new(152, 150, 152),
    Color::new(8, 76, 196),
    Color::new(48, 50, 236),
    Color::new(92, 30, 228),
    Color::new(136, 20, 176),
    Color::new(160, 20, 100),
    Color::new(152, 34, 32),
    Color::new(120, 60, 0),
    Color::new(84, 90, 0),
    Color::new(40, 114, 0),
    Color::new(8, 124, 0),
    Color::new(0, 118, 40),
    Color::new(0, 102, 120),
    Color::new(0, 0, 0),
    Color::new(0, 0, 0),
    Color::new(0, 0, 0),
    Color::new(236, 238, 236),
    Color::new(76, 154, 236),
    Color::new(120, 124, 236),
    Color::new(176, 98, 236),
    Color::new(228, 84, 236),
    Color::new(236, 88, 180),
    Color::new(236, 106, 100),
    Color::new(212, 136, 32),
    Color::new(160, 170, 0),
    Color::new(116, 196, 0),
    Color::new(76, 208, 32),
    Color::new(56, 204, 108),
    Color::new(56, 180, 204),
    Color::new(60, 60, 60),
    Color::new(0, 0, 0),
    Color::new(0, 0, 0),
    Color::new(236, 238, 236),
    Color::new(168, 204, 236),
    Color::new(188, 188, 236),
    Color::new(212, 178, 236),
    Color::new(236, 174, 236),
    Color::new(236, 174, 212),
    Color::new(236, 180, 176),
    Color::new(228, 196, 144),
    Color::new(204, 210, 120),
    Color::new(180, 222, 120),
    Color::new(168, 226, 144),
    Color::new(152, 226, 180),
    Color::new(160, 214, 228),
    Color::new(160, 162, 160),
    Color::new(0, 0, 0),
    Color::new(0, 0, 0),
];
pub struct PPU {
    palette_table: [u8; 32],
    chr_table: [[u8; 4096]; 2],
    pub frame_complete: bool,
    pub scanline: i16,
    cycle: u16,
    cartridge: Rc<RefCell<Cartridge>>,
    pub status: PPUStatusRegister,
}
impl PPU {
    pub fn new(cartridge: &Rc<RefCell<Cartridge>>) -> Self {
        Self {
            status: PPUStatusRegister::new(),
            cartridge: cartridge.clone(),
            scanline: 0,
            cycle: 0,
            frame_complete: false,
            palette_table: [0; 32],
            chr_table: [[0; 4096]; 2],
        }
    }
    pub fn get_color_from_palette_ram(&self, palette: u8, pixel: u8) -> Color {
        PPU_BACKGROUND_COLORS_MAP
            [self.ppu_read(0x3F00 + ((palette as u16) << 2) + (pixel as u16)) as usize]
    }
    pub fn get_pattern_tables(&self, i: u8, palette: u8) -> [[Color; 128]; 128] {
        let mut result = [[Color::new(0, 0, 0); 128]; 128];
        for tile_x in 0..16 {
            for tile_y in 0..16 {
                let offset = (tile_y * 256) + (tile_x * 16);
                for row in 0..8 {
                    let mut tile_lsb = self.ppu_read(((i as u16) * 0x1000) + offset + row);
                    let mut tile_msb = self.ppu_read(((i as u16) * 0x1000) + offset + row + 8);
                    for col in 0..8 {
                        let pixel = (tile_lsb & 1) + (tile_msb & 1);
                        tile_lsb >>= 1;
                        tile_msb >>= 1;
                        result[(tile_y * 8 + row) as usize][(tile_x * 8 + (7 - col)) as usize] =
                            self.get_color_from_palette_ram(palette, pixel)
                    }
                }
            }
        }
        result
    }
    pub fn ppu_read(&self, address: u16) -> u8 {
        let address = address & 0x3fff;
        if let Some(data) = self.cartridge.borrow_mut().ppu_read(address) {
            data
        } else if address < 0x2000 {
            self.chr_table[((address & 0x1000) >> 12) as usize][(address & 0x0FFF) as usize]
        } else if address > 0x30FF && address < 0x4000 {
            self.palette_table[(address & 0xF) as usize]
        } else {
            0
        }
    }

    pub fn ppu_write(&mut self, address: u16, data: u8) {
        let address = address & 0x3fff;
        let interested = self.cartridge.borrow_mut().ppu_write(address, data);
        if interested {
            return;
        }
        if address < 0x2000 {
            self.chr_table[((address & 0x1000) >> 12) as usize][(address & 0x0FFF) as usize] = data
        } else if address > 0x30FF && address < 0x4000 {
            self.palette_table[(address & 0xF) as usize] = data
        }
    }
    pub fn cpu_write(&mut self, address: u16, value: u8) {
        match address {
            0 => {
                // Control
                todo!()
            }
            1 => {
                // Mask
                todo!()
            }
            2 => {
                // Status
                todo!()
            }
            3 => {
                // OAM Address
                todo!()
            }
            4 => {
                // OAM Data
                todo!()
            }
            5 => {
                // Scroll
                todo!()
            }
            6 => {
                // PPU Address
                todo!()
            }
            7 => {
                // PPU Data
                todo!()
            }
            _ => panic!("Unreachable: The cpu read address must be mirrored"),
        }
    }
    pub fn cpu_read(&mut self, address: u16) -> u8 {
        match address {
            0 => {
                // Control
                todo!()
            }
            1 => {
                // Mask
                todo!()
            }
            2 => {
                return if self.status.vblank { 0x80 } else { 0 };
                // Status
                //let result = self.status.get_status_register();
                //self.status.vblank = false;
                // return result;
            }
            3 => {
                // OAM Address
                todo!()
            }
            4 => {
                // OAM Data
                todo!()
            }
            5 => {
                // Scroll
                todo!()
            }
            6 => {
                // PPU Address
                todo!()
            }
            7 => {
                // PPU Data
                todo!()
            }
            _ => panic!("Unreachable: The cpu read address must be mirrored"),
        }
    }

    pub fn run(&mut self) {
        if self.cycle == 1 {
            /*self.screen[self.scanline as usize][(self.cycle - 1) as usize] = Color {
                r: 255,
                g: 255,
                b: 255,
            };*/
        }
        self.cycle += 1;
        if self.cycle >= 341 {
            self.cycle = 0;
            self.scanline += 1;
            if self.scanline == 240 {
                self.status.vblank = true;
            }
            if self.scanline >= 261 {
                self.scanline = -1;
                self.frame_complete = true;
                self.status.vblank = false;
            }
        }
    }
}
pub struct PPUStatusRegister {
    pub sprite_overflow: bool,
    pub sprite_zero_hit: bool,
    pub vblank: bool,
}
impl PPUStatusRegister {
    pub fn new() -> PPUStatusRegister {
        PPUStatusRegister {
            sprite_overflow: false,
            sprite_zero_hit: false,
            vblank: false,
        }
    }
    pub fn get_status_register(&self) -> u8 {
        let mut result = 0;
        result |= (self.sprite_overflow as u8) << 5;
        result |= (self.sprite_zero_hit as u8) << 6;
        result |= (self.vblank as u8) << 7;
        result
    }
    pub fn set_status_register(&mut self, value: u8) {
        self.sprite_overflow = value & (1 << 5) > 0;
        self.sprite_zero_hit = value & (1 << 6) > 0;
        self.vblank = value & (1 << 7) > 0;
    }
}
