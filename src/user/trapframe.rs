
use alloc::boxed::Box;

use crate::arch::regs::Registers;

use super::task::Task;
#[derive(Default)]
#[repr(align(4096), C)]
pub struct TrapFrame {
	pub kernel_satp: usize,
	pub kernel_sp: usize,
	pub kernel_trap: usize,
	pub hartid: usize,
	pub satp: usize,
	pub regs: Registers,
	pub task: Option<*mut Task>
}
impl TrapFrame {
	pub fn new_box() -> Box<Self> {
		Box::new(Self::default())
	}
}
