#![no_std]
#![no_main]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(private_interfaces)]
#![allow(static_mut_refs)]
#![allow(unused_imports)]

extern crate SyscallAPI;
extern crate alloc;
extern crate sbi_rt;

mod asm_funcs;
mod config;
#[macro_use]
mod print;
mod arch;
mod driver;
mod lang;
mod mm;
mod sync;
mod test;
use alloc::boxed::Box;
use alloc::string::String;
use driver::uart::uart_device;
use user::scheduler;
use user::task::{Context, Task};
use user::trapframe::TrapFrame;
use xmas_elf::ElfFile;

mod user;

use core::mem;
use core::{alloc::Layout, arch::asm};

pub use crate::arch::shutdown;
use crate::arch::{get_tp, set_timer};
use crate::lang::memset;
use asm_funcs::*;
use mm::vm::{self, kvm_map, vm_map, PTE};
pub use mm::{alloc, dealloc};

use crate::{arch::trap::trap_init, mm::vm::vm_map_trampoline};

#[no_mangle]
pub static mut dtb_addr: usize = 0;

use crate::config::*;

#[no_mangle]
pub extern "C" fn kmain_early() {
	driver::uart::uart_device.init(uart_base_addr as usize);
	test::test_dynamic_function();
	success!("start kmain early init");
	mm::simple_allocator.init(ekernel as usize);
	mm::vm::init_kvm();
	success!("end kmain early init");
}

#[no_mangle]
pub extern "C" fn kmain() {
	trap_init();
	test::test_dynamic_function();
	success!("start kmain");
	unsafe {
		mm::buddy_allocator.init(mm::simple_allocator.get_control_range().1, 0x84000000);
	}
	mm::change_to_buddy();
	test::test_buddy();
	test_elf();
	shutdown();
}

fn test_elf() {
	let data = user::get_userapp_by_name("c").unwrap();
	let task = Task::from_elf(data);
	scheduler::add_task(task);
	scheduler::schedule_tasks();
}
