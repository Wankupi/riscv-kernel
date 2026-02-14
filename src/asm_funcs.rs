use crate::user::task::Context;

extern "C" {
	pub fn _trap_entry();
	pub fn stext();
	pub fn etext();
	pub fn srodata();
	pub fn erodata();
	pub fn sdata();
	pub static __rela_dyn_start: u8;
	pub static __rela_dyn_end: u8;
	pub fn edata();
	pub fn sbss();
	pub fn ebss();
	pub fn skernel();
	pub fn ekernel();
	pub fn _user_ret(satp: usize) -> !;
	pub fn boot_stack_top();
	pub fn _switch(from: &mut Context, to: &mut Context);
	pub fn _copy_u_s(src: *mut u8, dst: *mut u8, len: usize, user_satp: usize) -> isize;
	pub fn _wait_for_interrupt(sie: usize);
}

pub fn copy_u_s(src: *mut u8, dst: *mut u8, len: usize, user_satp: usize) -> isize {
	union Func {
		func: fn(*mut u8, *mut u8, usize, usize) -> isize,
		addr: usize,
	}
	let func = Func {
		addr: _copy_u_s as usize - _trap_entry as usize + 0xffffffff_ffff_f000,
	};
	unsafe { (func.func)(src, dst, len, user_satp) }
}
