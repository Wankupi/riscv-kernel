use core::arch::asm;
use alloc::boxed::Box;

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
pub extern "C" fn kernel_trap_entry() {
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

pub struct TrapFrame {
	pub kernel_satp: usize,
	pub kernel_sp: usize,
	pub kernel_trap: usize,
	pub hartid: usize,
	pub satp: usize,
	pub regs: [usize; 31],
	pub pc: usize,
}
impl TrapFrame {
	pub fn new() -> Self {
		TrapFrame {
			kernel_satp: 0,
			kernel_sp: 0,
			kernel_trap: 0,
			hartid: 0,
			regs: [0; 31],
			pc: 0,
			satp: 0
		}
	}
	pub fn new_box() -> Box<Self> {
		Box::new(Self::new())
	}
}
struct TaskStruct {
	pid: i32,
	tid: i32,
	uid: i32,
	trap_frame: Box<TrapFrame>,
}


impl TaskStruct {
	pub fn new() -> Self {
		TaskStruct {
			pid: 0,
			tid: 0,
			uid: 0,
			trap_frame: TrapFrame::new_box()
		}
	}
	pub fn new_box() -> Box<Self> {
		Box::new(Self::new())
	}
}