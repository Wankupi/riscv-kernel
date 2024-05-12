
use alloc::boxed::Box;

use crate::arch::regs::Registers;
#[derive(Default)]
#[repr(align(4096))]
pub struct TrapFrame {
	pub kernel_satp: usize,
	pub kernel_sp: usize,
	pub kernel_trap: usize,
	pub hartid: usize,
	pub satp: usize,
	pub regs: Registers,
}
impl TrapFrame {
	pub fn new_box() -> Box<Self> {
		Box::new(Self::default())
	}
}

