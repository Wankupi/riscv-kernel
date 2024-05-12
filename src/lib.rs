#![no_std]
#![no_main]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(private_interfaces)]

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
use alloc::string::String;
use user::trapframe::TrapFrame;
use xmas_elf::ElfFile;

mod user;

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
	// for i in 0..10 {
	// 	log!("time = {:x}", arch::get_clock());
	// }
	// test_user();
	test_elf();
	shutdown();
}

fn test_user() {
	// let _vt = mm::vm::VirtMapPage::create();
	// let vt = unsafe { &mut *_vt };
	// let mut _tf = TrapFrame::new_box();
	// let tf = _tf.as_mut();
	// tf.kernel_satp = vm::get_kernel_satp();
	// tf.kernel_sp = boot_stack_top as usize;
	// tf.kernel_trap = arch::trap::kernel_trap_entry as usize;
	// tf.hartid = get_tp();
	// memset(&mut tf.regs as *mut _ as *mut u8, 0, 32 * 8);
	// // set sp = 0xeeeeeeee_00000000
	// tf.regs.sp_x2 = 0xeeeeeeee_00000000;
	// tf.regs.pc = 0x1_00000000;
	// tf.satp = (_vt as usize >> 12) | (8 << 60);
	// kvm_map(
	// 	0xffffffff_ffff_e000,
	// 	tf as *const TrapFrame as usize,
	// 	4096,
	// 	PTE::RW,
	// );
	// vm_map_trampoline(vt);
	// let text = mm::alloc(Layout::from_size_align(4096, 4096).unwrap());
	// vm_map(vt, tf.regs.pc, text as usize, 4096, PTE::RX | PTE::U);
	// vm_map(vt, uart_base_addr, uart_base_addr, 4096, PTE::RW | PTE::U);
	// let program: [u32; 4] = [
	// 	0x10000537, // lui a0,0x10000
	// 	0x0310059b, // addiw a1,zero,0x31
	// 	0x00b50023, // sb a1,0(a0)
	// 	0xbfd5,     // j 0
	// ];
	// for i in 0..program.len() {
	// 	unsafe {
	// 		(text as *mut u32).add(i).write(program[i]);
	// 	}
	// }
	// log!("done write content");

	// let offset = _user_ret as usize - _trap_entry as usize;
	// let user_ret_func = Func { v: offset + 0xffffffff_ffff_f000 };
	// set_timer();
	// let sie = 1 << 9 | 1 << 5 | 1 << 1;
	// unsafe {
	// 	asm!("csrw sie, {}", in(reg) sie);
	// 	asm!("csrw stvec, {}", in(reg) (0xffffffff_ffff_f000 as usize));
	// 	// asm!("jr {}", in(reg) user_ret_func);
	// 	(user_ret_func.func)(0);
	// }
}

union Func {
	v: usize,
	func: fn(usize) -> !,
}

fn test_elf() {
	let start = elf1_start as usize;
	let end = elf1_end as usize;
	let data = unsafe { core::slice::from_raw_parts(start as *const u8, end - start) };
	let elf = ElfFile::new(data).unwrap();

	let header = &elf.header;
	if header.pt1.magic != [0x7f, 0x45, 0x4c, 0x46] {
		error!("not a elf file");
		return;
	}
	success!("magic check pass");

	for header in elf.program_iter() {
		if header.get_type().unwrap() != xmas_elf::program::Type::Load {
			continue;
		}
		// let s = header.virtual_addr() as usize;
		// let size = header.mem_size();
		// let offset = header.offset();
		// println!("s = {:x}, size = {:x}", s, size);
		println!("{}", header);
	}

	for section in elf.section_iter() {
		println!("{}", section);
	}
}
