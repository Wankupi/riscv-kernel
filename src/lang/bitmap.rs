use core::mem::size_of;

use super::memset;


#[derive(Copy, Clone)]
pub struct Bitmap {
	data: *mut usize,
	size: usize,
}

impl Bitmap {
	pub const fn new() -> Self {
		Self { data: 0 as *mut usize, size: 0 }
	}
	pub fn init(&mut self, data: *mut usize, size: usize) {
		self.data = data;
		self.size = size;
		memset(data as *mut u8, 0, size * size_of::<usize>());
	}
	pub fn get(&self, index: usize) -> bool {
		let word_index = index / core::mem::size_of::<usize>();
		let bit_index = index & (core::mem::size_of::<usize>() - 1);
		unsafe { self.data.add(word_index).read() & (1 << bit_index) != 0 }
	}
	pub fn set(&self, index: usize) {
		let word_index = index / core::mem::size_of::<usize>();
		let bit_index = index & (core::mem::size_of::<usize>() - 1);
		let old_value = unsafe { self.data.add(word_index).read() };
		let new_value = old_value | (1 << bit_index);
		unsafe { self.data.add(word_index).write(new_value) }
	}
	pub fn toggle(&self, index: usize) {
		let word_index = index / core::mem::size_of::<usize>();
		let bit_index = index & (core::mem::size_of::<usize>() - 1);
		let old_value = unsafe { self.data.add(word_index).read() };
		let new_value = old_value ^ (1 << bit_index);
		unsafe { self.data.add(word_index).write(new_value) }
	}
}
