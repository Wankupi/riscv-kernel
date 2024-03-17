mod uart_regs {
	pub const rbr: usize = 0;
	pub const ier: usize = 1;
	pub const iir: usize = 2;
	pub const lcr: usize = 3;
	pub const mcr: usize = 4;
	pub const lsr: usize = 5;
	pub const msr: usize = 6;
	pub const scr: usize = 7;
	pub const thr: usize = 0;
	pub const fcr: usize = 2;
}
use core::cell::UnsafeCell;

use uart_regs::*;

pub struct UartRaw {
	base: UnsafeCell<usize>,
}

impl UartRaw {
	pub const fn new(device_addr: usize) -> UartRaw {
		UartRaw {
			base: UnsafeCell::new(device_addr),
		}
	}
	fn store(&self, reg: usize, val: u8) {
		unsafe { ((*self.base.get() + reg) as *mut u8).write_volatile(val) }
	}
	fn load(&self, reg: usize) -> u8 {
		unsafe { ((*self.base.get() + reg) as *mut u8).read_volatile() }
	}
	pub fn init(&self, addr: usize) {
		unsafe { *self.base.get() = addr }
		return; // sbi will initialize the uart
		self.store(ier, 0);
		self.store(lcr, 1 << 7);
		self.store(thr, 0x03);
		self.store(ier, 0);
		self.store(lcr, 0b11);
		self.store(fcr, 1 | (1 << 1) | (1 << 2));
		self.store(ier, 1 | (1 << 1));
	}
	pub fn write(&self, data: u8) {
		const LSR_THR_EMPTY: u8 = 1 << 5;
		while (self.load(lsr) & LSR_THR_EMPTY) == 0 {}
		self.store(thr, data);
	}
	pub fn read(&self) -> u8 {
		const LSR_DATA_READY: u8 = 1;
		while (self.load(lsr) & LSR_DATA_READY) == 0 {}
		self.load(rbr)
	}
}

unsafe impl Sync for UartRaw {}

pub static uart_device: UartRaw = UartRaw::new(0);
