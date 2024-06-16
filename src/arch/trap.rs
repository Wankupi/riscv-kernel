use crate::{
	asm_funcs::*,
	lang::memset,
	mm::vm::{kvm_map, PTE},
	user::{scheduler::yield_this, syscall::syscall, task::Task, trapframe::TrapFrame},
	PAGE_SIZE,
};
use alloc::{boxed::Box, task};
use core::{
	arch::asm,
	mem::{size_of, size_of_val},
};

use crate::{arch::shutdown, print::printk};

use super::set_timer;

fn set_trap(addr: usize) {
	unsafe { asm!("csrw stvec, {}", in(reg) addr) }
}
pub fn trap_init() {
	set_trap(_trap_entry as usize);
}

union Func {
	v: usize,
	func: fn(usize) -> !,
}

pub fn run_user() {
	let offset = _user_ret as usize - _trap_entry as usize;
	let user_ret_func = Func {
		v: offset + 0xffffffff_ffff_f000,
	};
	set_timer();
	let sie = 1 << 9 | 1 << 5 | 1 << 1;
	unsafe {
		asm!("csrw sie, {}", in(reg) sie);
		asm!("csrw stvec, {}", in(reg) (0xffffffff_ffff_f000 as usize));
		(user_ret_func.func)(0);
	}
	error!("Unreachable code");
	loop {}
}

fn unknown_error(cause: usize) -> ! {
	let pc: usize;
	unsafe { asm!("csrr {}, sepc", out(reg) pc) }
	let trampoline = unsafe { &mut *(0xffffffff_ffff_e000 as *mut TrapFrame) };
	let task = unsafe { &mut *trampoline.task.unwrap() };
	let pid = task.process.pid;
	println!("[{}] unknown trap. pc: {:x} cause: {:x}", pid, pc, cause);
	shutdown();
}

#[no_mangle]
pub extern "C" fn kernel_trap_entry() {
	let trampoline = unsafe { &mut *(0xffffffff_ffff_e000 as *mut TrapFrame) };
	let task = unsafe { &mut *trampoline.task.unwrap() };
	let mut cause: usize;
	unsafe { asm!("csrr {}, scause", out(reg) cause) }
	let interrupt = (cause >> 63) == 1;
	cause &= (1 << 63) - 1;
	if interrupt {
		match cause {
			5 => {
				printk(b"\ntimer interrupt\n");
				yield_this();
			}
			_ => unknown_error(cause),
		}
	} else {
		match cause {
			8 => syscall(task),
			_ => unknown_error(cause),
		}
	}
	set_timer();
	return;
}
