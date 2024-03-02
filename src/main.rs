#![no_std]
#![no_main]

mod arch;
mod lang_items;

use core::arch::global_asm;

global_asm!(include_str!("entry.asm"));

#[no_mangle]
pub extern "C" fn kmain() -> ! {
	let s : &'static str = "Hello, world!\n";
	arch::printk(s);
	arch::shutdown();
}
