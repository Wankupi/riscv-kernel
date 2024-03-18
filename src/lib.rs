#![no_std]
#![no_main]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(unreachable_code)]

extern crate alloc;

mod allocator;
mod arch;
mod driver;
mod lang_items;
mod mm;
mod print;

pub use crate::arch::shutdown;

use crate::arch::trap::trap_init;


#[no_mangle]
pub static mut dtb_addr: usize = 0;

extern "C" {
	fn ekernel();
	fn uart_base_addr();
}

use allocator::SimpleAllocator;
#[global_allocator]
static ALLOCATOR: SimpleAllocator = SimpleAllocator::new();

#[no_mangle]
pub extern "C" fn kmain() {
	driver::uart::uart_device.init(uart_base_addr as usize);
	success!("start kernel");
	trap_init();
	ALLOCATOR.init(ekernel as usize);
	mm::vm::init_kvm();
	mm::vm::kvm_start();
	shutdown();
}
