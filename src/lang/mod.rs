mod panic;
mod unsafe_array;
pub use unsafe_array::*;

mod bitmap;
pub use bitmap::*;

mod arcbox;


pub fn memset(dest: *mut u8, value: u8, size: usize) {
	let mut i = 0;
	while i < size {
		unsafe {
			dest.add(i).write(value);
		}
		i += 1;
	}
}


pub fn memcpy(dest: *mut u8, src: *const u8, size: usize) {
	let mut i = 0;
	while i < size {
		unsafe {
			dest.add(i).write(src.add(i).read());
		}
		i += 1;
	}
}
