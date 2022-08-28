use std::cell::RefCell;
use std::f64;
use std::{rc::Rc, sync::*};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::CanvasRenderingContext2d;
use yane_core::*;
#[wasm_bindgen] extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen(start)]
pub fn start(){
    console_error_panic_hook::set_once();
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();
    let cartridge = Arc::new(Mutex::new(get_cartridge()));
    let ppu = Arc::new(Mutex::new(PPU::new(cartridge.clone())));
    let memory = Memory::new(cartridge.clone(), ppu.clone());
    let mut cpu = CPU::new(Rc::new(memory));
    let mut clock_counter = 0;
    cpu.init();
    log("Everything's working, Everything's twerking.");
    let f = Rc::new(RefCell::new(None));
    let g = f.clone();
    *g.borrow_mut() = Some(Closure::new(move || {

        clock_counter = clock(clock_counter, &mut cpu, ppu.clone());
        context.set_fill_style(&"#FFFFFF".into());
        context.fill_rect(0.0, 0.0, 256.0, 240.0);
        let ppu = ppu.lock().unwrap();
        draw_chr_memory(&context, 0, 1, 0, 0, &ppu);
        draw_chr_memory(&context, 1, 1, 0, 128, &ppu);
        request_animation_frame(f.borrow().as_ref().unwrap());
    }));
    request_animation_frame(g.borrow().as_ref().unwrap());
}
fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}
fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}
fn draw_chr_memory(window: &CanvasRenderingContext2d, i: u8, palette: u8, offset_x: i32, offset_y: i32, ppu: &PPU){
    let colors = ppu.get_pattern_tables(i, palette);
        for x in 0..128i32 {
            for y in 0..128i32 {
                let color = colors[y as usize][x as usize];
                window.set_fill_style(&format!("#{:02x}{:02x}{:02x}", color.r, color.g, color.b).into());
                window.fill_rect((x + offset_x) as f64, (y + offset_y) as f64, 1.0, 1.0);
            }
        }
}
fn clock(clock_counter: u32, cpu: &mut CPU, ppu: Arc<Mutex<PPU>>) -> u32 {
    if (clock_counter % 3) == 0 {
        cpu.clock();
    }
    ppu.lock().unwrap().run();
    clock_counter + 1
}

fn get_cartridge() -> Cartridge {
    use std::io::Read;
    let bytes = include_bytes!("../../test-roms/nestest.nes");
    Cartridge::from_read(Box::new(bytes.take(bytes.len() as u64)))
}
