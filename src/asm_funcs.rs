use crate::user::task::Context;

extern "C" {
	pub fn _trap_entry();
	pub fn stext();
	pub fn etext();
	pub fn srodata();
	pub fn erodata();
	pub fn sdata();
	pub fn edata();
	pub fn sbss();
	pub fn ebss();
	pub fn skernel();
	pub fn ekernel();
	pub fn _user_ret(satp: usize) -> !;
	pub fn boot_stack_top();
	pub fn _switch(from: &mut Context, to: &mut Context);
}
