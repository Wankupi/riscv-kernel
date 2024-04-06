#![no_std]
#![no_main]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(unreachable_code)]

extern crate alloc;

mod arch;
mod driver;
mod lang;
mod mm;
mod print;
mod sync;
pub use mm::alloc;
pub use crate::arch::shutdown;

use crate::arch::trap::trap_init;

#[no_mangle]
pub static mut dtb_addr: usize = 0;

extern "C" {
	fn ekernel();
	fn uart_base_addr();
}


#[no_mangle]
pub extern "C" fn kmain_early() {
	driver::uart::uart_device.init(uart_base_addr as usize);
	success!("start kmain early init");
	mm::simple_allocator.init(ekernel as usize);
	mm::vm::init_kvm();
}


#[no_mangle]
pub extern "C" fn kmain() {
	success!("start kmain");
	trap_init();
	
	unsafe {
		mm::buddy_allocator.init(mm::simple_allocator.get_current_pos(), 0x88000000);
	}
	shutdown();
}
