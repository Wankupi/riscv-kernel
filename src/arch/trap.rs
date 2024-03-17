use core::arch::asm;

use crate::arch::shutdown;
use crate::print;

fn set_trap(addr: usize) {
	unsafe { asm!("csrw stvec, {}", in(reg) addr) }
}

extern "C" {
	fn _trap_entry();
}

pub fn trap_init() {
	set_trap(_trap_entry as usize);
}

#[no_mangle]
extern "C" fn solve_trap() {
	let pc: usize;
	unsafe { asm!("csrr {}, sepc", out(reg) pc) }
	print!("Trap! from addr = {:x}\n", pc);
	shutdown();
}
