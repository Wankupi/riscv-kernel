use core::arch::asm;

use crate::{arch::shutdown, print::printk};

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
extern "C" fn kernel_trap_entry() {
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
}
