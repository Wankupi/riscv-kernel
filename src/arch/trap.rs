use crate::{
	asm_funcs::*,
	lang::memset,
	mm::vm::{kvm_map, PTE},
	user::{scheduler::{yield_this}, task::Task, trapframe::TrapFrame},
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



#[no_mangle]
pub extern "C" fn kernel_trap_entry() {
	let unknown_error = || -> ! {
		let pc: usize;
		unsafe { asm!("csrr {}, sepc", out(reg) pc) }
		// error!("Trap! from addr = {:x}", pc);
		let mut str: [u8; 40] = [0; 40];
		let pre = b"Trap! from addr = ";
		let mut len = pre.len();
		for i in 0..pre.len() {
			str[i] = pre[i];
		}
		for i in 0..16 {
			let v: u8 = (pc >> (4 * (15 - i)) & 0xf) as u8;
			str[len + i] = if v < 10 { v + b'0' } else { v - 10 + b'a' };
		}
		len += 16;
		str[len] = 0;
		printk(&str);
		shutdown();
	};
	let trampoline = unsafe { &mut *(0xffffffff_ffff_e000 as *mut TrapFrame) };
	let task = unsafe { &mut *trampoline.task.unwrap() };
	let mut cause: usize;
	unsafe { asm!("csrr {}, scause", out(reg) cause) }
	if cause >> 63 == 1 {
		cause ^= 1 << 63;
		match cause {
			5 => {
				printk(b"\ntimer interrupt\n");
				yield_this(task);
				set_timer();
				return;
			}
			_ => unknown_error(),
		}
	} else {
		unknown_error();
	}
}
