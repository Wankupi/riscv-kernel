#![no_std]
#![no_main]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(unreachable_code)]

extern crate alloc;
mod config;
#[macro_use]
mod print;
mod arch;
mod driver;
mod lang;
mod mm;
mod sync;
mod test;
pub use mm::{alloc, dealloc};
pub use crate::arch::shutdown;

use crate::arch::trap::trap_init;

#[no_mangle]
pub static mut dtb_addr: usize = 0;

extern "C" {
	fn ekernel();
}

use crate::config::*;

#[no_mangle]
pub extern "C" fn kmain_early() {
	driver::uart::uart_device.init(uart_base_addr as usize);
	success!("start kmain early init");
	mm::simple_allocator.init(ekernel as usize);
	mm::vm::init_kvm();
	success!("end kmain early init");
}

#[no_mangle]
pub extern "C" fn kmain() {
	trap_init();
	success!("start kmain");
	mm::simple_allocator.dbg_report();
	unsafe {
		mm::buddy_allocator.init(mm::simple_allocator.get_control_range().1, 0x88000000);
	}
	mm::simple_allocator.dbg_report();
	mm::change_to_buddy();
	test::test_buddy();

	shutdown();
}
