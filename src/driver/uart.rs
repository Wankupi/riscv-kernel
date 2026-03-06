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

use crate::driver::fdt;

pub struct UartRaw {
	base: UnsafeCell<*const u8>,
	io_width: UnsafeCell<usize>,
	reg_shift: UnsafeCell<usize>,
	buffer: [u8; 1024],
	len: usize,
}

impl UartRaw {
	pub const fn new(device_addr: *const u8) -> UartRaw {
		UartRaw {
			base: UnsafeCell::new(device_addr),
			io_width: UnsafeCell::new(1),
			reg_shift: UnsafeCell::new(0),
			buffer: [0; 1024],
			len: 0,
		}
	}
	fn reg_addr(&self, reg: usize) -> *const u8 {
		unsafe { (*self.base.get()).wrapping_add(reg << *self.reg_shift.get()) }
	}
	fn store(&self, reg: usize, val: u8) {
		let addr = self.reg_addr(reg);
		let io_width = unsafe { *self.io_width.get() };
		unsafe {
			match io_width {
				1 => (addr as *mut u8).write_volatile(val),
				2 => (addr as *mut u16).write_volatile(val as u16),
				4 => (addr as *mut u32).write_volatile(val as u32),
				_ => (addr as *mut u8).write_volatile(val),
			}
		}
	}
	fn load(&self, reg: usize) -> u8 {
		let addr = self.reg_addr(reg);
		let io_width = unsafe { *self.io_width.get() };
		unsafe {
			match io_width {
				1 => (addr as *mut u8).read_volatile(),
				2 => (addr as *mut u16).read_volatile() as u8,
				4 => (addr as *mut u32).read_volatile() as u8,
				_ => (addr as *mut u8).read_volatile(),
			}
		}
	}
	pub fn init(&self, addr: *const u8) {
		self.init_with_config(addr, 1, 0);
	}
	pub fn init_with_config(&self, addr: *const u8, io_width: usize, reg_shift: usize) {
		unsafe { *self.base.get() = addr }
		unsafe {
			*self.io_width.get() = io_width;
			*self.reg_shift.get() = reg_shift;
		}
		// return; // sbi will initialize the uart
		// self.store(ier, 0);
		// self.store(lcr, 1 << 7);
		// self.store(thr, 0x03);
		// self.store(lcr, 0b11);
		// self.store(fcr, 1 | (1 << 1) | (1 << 2));
		// self.store(ier, 1 | (1 << 1));
		self.store(ier, 1);
	}
	pub fn base_addr(&self) -> usize {
		unsafe { *self.base.get() as usize }
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

pub static mut uart_device: UartRaw = UartRaw::new(core::ptr::null());

pub fn init_stdout_from_fdt() {
	let fdt = fdt::get_fdt();
	let stdout = fdt.find_node("serial0").unwrap();
	let mem_region = stdout.reg().unwrap().next().unwrap();
	let io_width = stdout.property("reg-io-width").map_or(1, |p| {
		let bytes: [u8; 4] = p.value.try_into().unwrap();
		u32::from_be_bytes(bytes) as usize
	});
	let reg_shift = stdout.property("reg-shift").map_or(0, |p| {
		let bytes: [u8; 4] = p.value.try_into().unwrap();
		u32::from_be_bytes(bytes) as usize
	});
	unsafe {
		uart_device.init_with_config(mem_region.starting_address, io_width, reg_shift);
	}
}
