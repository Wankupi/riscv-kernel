mod panic;
mod unsafe_array;
pub use unsafe_array::*;

mod bitmap;
pub use bitmap::*;

mod arcbox;
use crate::asm_funcs;

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

pub fn fix_rela_dyn(base_addr: usize, rela_offset: usize) {
	#[repr(C)]
	struct RelaDynEntry {
		r_offset: usize,
		r_info: usize,
		r_addend: isize,
	}
	let rela_start = core::ptr::addr_of!(asm_funcs::__rela_dyn_start) as *const RelaDynEntry;
	let rela_end = core::ptr::addr_of!(asm_funcs::__rela_dyn_end) as *const RelaDynEntry;
	unsafe {
		let mut p = rela_start;
		while p < rela_end {
			let e = &*p;
			let to_write = (e.r_offset + base_addr) as *mut usize;
			if e.r_info == 0x3 {
				*to_write = base_addr + rela_offset + e.r_addend as usize;
			}
			p = p.add(1);
		}
	}
}
