use std::cell::Ref;
use std::cell::RefCell;
use std::cell::RefMut;
use std::rc::Rc;
use crate::{PPU, CPU, Cartridge, Memory};
type Wrapped<T> = Rc<RefCell<T>>;
macro_rules! getter {
	($var:ident, $mut_var:ident, $type:ty) => {
		pub fn $mut_var(&mut self) -> RefMut<$type> {
			self.$var.borrow_mut()
		}
		pub fn $var(&self) -> Ref<$type> {
			self.$var.borrow()
		}
	};
}
pub struct NES {
	pub cpu: Wrapped<CPU>,
	pub ppu: Wrapped<PPU>,
	pub cartridge: Wrapped<Cartridge>,
	pub memory: Wrapped<Memory>,
	clock_count: u8
}
impl NES {
	getter!(memory, mut_memory, Memory);
	getter!(ppu, mut_ppu, PPU);
	getter!(cartridge, mut_cartridge, Cartridge);
	getter!(cpu, mut_cpu, CPU);
	pub fn clock(&mut self){
		self.mut_ppu().run();
		if self.clock_count % 3 == 0 {
			self.mut_cpu().clock();
		}
		self.clock_count += 1;
	}
	pub fn new(cartridge: Cartridge) -> Self {
		let cartridge = wrap(cartridge);
		let ppu = wrap(PPU::new(cartridge.clone()));
		let memory = wrap(Memory::new(cartridge.clone(), ppu.clone()));
		let mut cpu = CPU::new(memory.clone());
		cpu.init();
		Self {
			cartridge,
			ppu,
			memory,
			cpu: wrap(cpu),
			clock_count: 0,
		}
	}
}
fn wrap<T>(v: T) -> Wrapped<T> {
	return Rc::new(RefCell::new(v));
}
