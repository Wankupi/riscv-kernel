#![no_std]
#![no_main]

extern crate alloc;

mod allocator;
mod arch;
mod driver;
mod lang_items;
mod mm;
mod print;

pub use crate::arch::shutdown;

use crate::arch::trap::trap_init;

use allocator::SimpleAllocator;
use mm::declare::{PhysAddr, PhysPageNum};

#[no_mangle]
pub static mut dtb_addr: usize = 0;

extern "C" {
	fn skernel();
	fn ekernel();
	fn uart_base_addr();
}

#[global_allocator]
static ALLOCATOR: SimpleAllocator = SimpleAllocator::new();

#[no_mangle]
pub extern "C" fn kmain() {
	driver::uart::uart_device.init(uart_base_addr as usize);
	print!("\x1b[32mKernel start.\x1b[0m\n");
	print!("hah");
	// trap_init();
	// ALLOCATOR.init(ekernel as usize);
	// print!("start init vm\n");
	// mm::declare::init_vm(
	// 	PhysPageNum::from(PhysAddr::from(skernel as usize)),
	// 	PhysPageNum::from(PhysAddr::from(ekernel as usize)),
	// );
	shutdown();
}
