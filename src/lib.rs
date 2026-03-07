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

use alloc::boxed::Box;
use alloc::string::String;
use driver::uart::uart_device;
use user::scheduler;
use user::task::{Context, Task};
use user::trapframe::TrapFrame;
use xmas_elf::ElfFile;

mod user;

use core::mem;
use core::ptr::addr_of;
use core::{alloc::Layout, arch::asm};

pub use crate::arch::shutdown;
use crate::arch::{get_tp, set_timer};
use crate::driver::fdt;
use crate::mm::vm::kvm_config;
use crate::print::{print_hex, printk};
use asm_funcs::*;
use mm::vm::{self, kvm_map, vm_map, PTE};
pub use mm::{alloc, dealloc};

use crate::config::*;
use crate::{arch::trap::trap_init, mm::vm::vm_map_trampoline};

#[no_mangle]
pub extern "C" fn kmain_early(core_id: usize, dtb_addr: *const u8) {
	fdt::init_fdt_early(dtb_addr);
	driver::uart::init_stdout_from_fdt();
	let base_addr = addr_of!(kernel_load_base) as usize;
	lang::fix_rela_dyn(base_addr, 0);
	success!("start kmain early init on core {}", core_id);
	mm::simple_allocator.init(
		(ekernel as *const () as usize + PAGE_SIZE - 1) & !(PAGE_SIZE - 1),
		PAGE_SIZE * 2,
	);
	fdt::fdt_move_to_owned();
	mm::allocator::pre_alloc_buddy();
	mm::vm::init_kvm();
	success!("end kmain early init");
	lang::fix_rela_dyn(base_addr, unsafe { kvm_config.v2p_offset_text });
}

#[no_mangle]
pub extern "C" fn kmain() {
	trap_init();
	success!("start kmain");
	fdt::init_fdt();
	mm::change_to_buddy();
	mm::simple_allocator.dbg_report();
	IPC::msg::init();
	irq::plic_init();
	println!("hello");
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
