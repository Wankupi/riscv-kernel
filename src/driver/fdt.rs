use fdt::Fdt;

use alloc::alloc::alloc;
use core::alloc::Layout;
use core::ptr::{copy, null};

use crate::driver::uart::uart_device;
use crate::mm::{self, vm};

static mut global_fdt: Option<Fdt<'static>> = None;
static mut _dtb_ptr: *const u8 = null();

pub fn init_fdt_early(dtb_addr: *const u8) {
	unsafe {
		assert!(global_fdt.is_none());
		_dtb_ptr = dtb_addr;
		global_fdt = Some(Fdt::from_ptr(dtb_addr).unwrap());
	}
}

pub fn init_fdt() {
	unsafe {
		_dtb_ptr = _dtb_ptr.wrapping_add(vm::kvm_config.v2p_offset_text);
		global_fdt = Some(Fdt::from_ptr(_dtb_ptr).unwrap());
	}
}

pub fn fdt_move_to_owned() {
	unsafe {
		assert!(!_dtb_ptr.is_null());
		let fdt = global_fdt.as_ref().unwrap();
		let total_size = fdt.total_size();
		let layout = Layout::from_size_align(total_size, 8).unwrap();
		let dst = mm::alloc(layout);
		assert!(!dst.is_null());
		copy(_dtb_ptr, dst, total_size);
		_dtb_ptr = dst;
		global_fdt = Some(Fdt::from_ptr(dst).unwrap());
	}
}

pub fn get_fdt() -> &'static Fdt<'static> {
	unsafe { global_fdt.as_ref().unwrap() }
}
