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
	buffer: [u8; 1024],
	len: usize,
}

impl UartRaw {
	pub const fn new(device_addr: usize) -> UartRaw {
		UartRaw {
			base: UnsafeCell::new(device_addr),
			buffer: [0; 1024],
			len: 0,
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
		// return; // sbi will initialize the uart
		// self.store(ier, 0);
		// self.store(lcr, 1 << 7);
		// self.store(thr, 0x03);
		// self.store(lcr, 0b11);
		// self.store(fcr, 1 | (1 << 1) | (1 << 2));
		// self.store(ier, 1 | (1 << 1));
		self.store(ier, 1);
	}
	pub fn write(&self, data: u8) {
		const LSR_THR_EMPTY: u8 = 1 << 5;
		while (self.load(lsr) & LSR_THR_EMPTY) == 0 {}
		self.store(thr, data);
	}
	pub fn read(&mut self) -> u8 {
		if self.len > 0 {
			let r = self.buffer[0];
			for i in 0..self.len - 1 {
				self.buffer[i] = self.buffer[i + 1];
			}
			self.len -= 1;
			return r;
		}
		const LSR_DATA_READY: u8 = 1;
		while (self.load(lsr) & LSR_DATA_READY) == 0 {}
		let data = self.load(rbr);
		self.write(data);
		data
	}
	pub fn device_ready(&mut self) {
		const LSR_DATA_READY: u8 = 1;
		while (self.load(lsr) & LSR_DATA_READY) != 0 {
			let data = self.load(rbr);
			self.buffer[self.len] = data;
			self.len += 1;
			self.write(data);
		}
	}
}

unsafe impl Sync for UartRaw {}

pub static mut uart_device: UartRaw = UartRaw::new(0);
