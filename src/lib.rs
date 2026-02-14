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
mod IPC;
mod arch;
mod driver;
mod irq;
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
use crate::mm::vm::kvm_config;
use crate::print::{print_hex, printk};
use asm_funcs::*;
use mm::vm::{self, kvm_map, vm_map, PTE};
pub use mm::{alloc, dealloc};

use crate::{arch::trap::trap_init, mm::vm::vm_map_trampoline};

#[no_mangle]
pub static mut dtb_addr: usize = 0;

use crate::config::*;



fn fix_rela_dyn(base_addr: usize) {
	#[repr(C)]
	struct RelaDynEntry {
		r_offset: usize,
		r_info: usize,
		r_addend: isize,
	}
	let rela_start = core::ptr::addr_of!(__rela_dyn_start) as usize;
	let rela_end = core::ptr::addr_of!(__rela_dyn_end) as usize;
	unsafe {
		let mut p = rela_start as *const RelaDynEntry;
		while (p as usize) < rela_end {
			let e = &*p;
			let to_write = e.r_offset as *mut usize;
			if e.r_info == 0x3 {
				*to_write = base_addr + e.r_addend as usize;
			}
			p = p.add(1);
		}
	}
}

#[no_mangle]
pub extern "C" fn kmain_early() {
	unsafe { driver::uart::uart_device.init(uart_base_addr as usize) };
	fix_rela_dyn(0x00000000);
	success!("start kmain early init");
	mm::simple_allocator.init(ekernel as usize);
	mm::vm::init_kvm();
	success!("end kmain early init");
	fix_rela_dyn(unsafe { kvm_config.v2p_offset_text });
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
	// test::test_buddy();
	IPC::msg::init();
	irq::plic_init();
	println!("hello");
	// test_elf();
	shutdown();
}

fn test_elf() {
	let data = user::get_userapp_by_name("A_print_1").unwrap();
	let task = Task::from_elf(data);
	scheduler::add_task(task);
	let data = user::get_userapp_by_name("A_print_1").unwrap();
	let mut task = Task::from_elf(data);
	task.process.trapframe.regs.tp_x4 = 1;
	scheduler::add_task(task);
	scheduler::schedule_tasks();
}
